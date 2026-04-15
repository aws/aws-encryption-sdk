/* Copyright Amazon.com Inc. or its affiliates. All Rights Reserved. */
/* SPDX-License-Identifier: Apache-2.0 */

#include "config.h"

#include <errno.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <yaml.h>

/* ---- internal helpers --------------------------------------------------- */

static int64_t *parse_int64_sequence(yaml_parser_t *parser, size_t *out_count) {
    yaml_event_t event;
    int64_t *values = NULL;
    size_t count = 0;
    size_t capacity = 8;

    values = malloc(capacity * sizeof(int64_t));
    if (!values) return NULL;

    /* consume SEQUENCE_START */
    if (!yaml_parser_parse(parser, &event)) { free(values); return NULL; }
    if (event.type != YAML_SEQUENCE_START_EVENT) {
        yaml_event_delete(&event);
        free(values);
        return NULL;
    }
    yaml_event_delete(&event);

    while (1) {
        if (!yaml_parser_parse(parser, &event)) { free(values); return NULL; }
        if (event.type == YAML_SEQUENCE_END_EVENT) {
            yaml_event_delete(&event);
            break;
        }
        if (event.type == YAML_SCALAR_EVENT) {
            if (count >= capacity) {
                capacity *= 2;
                int64_t *tmp = realloc(values, capacity * sizeof(int64_t));
                if (!tmp) { free(values); yaml_event_delete(&event); return NULL; }
                values = tmp;
            }
            const char *str = (const char *)event.data.scalar.value;
            char *endptr;
            errno = 0;
            int64_t val = strtoll(str, &endptr, 10);
            if (errno != 0 || endptr == str || *endptr != '\0') {
                fprintf(stderr, "Invalid integer in config: '%s'\n", str);
                yaml_event_delete(&event);
                continue;
            }
            values[count++] = val;
        }
        yaml_event_delete(&event);
    }

    *out_count = count;
    return values;
}

static int *parse_int_sequence(yaml_parser_t *parser, size_t *out_count) {
    yaml_event_t event;
    int *values = NULL;
    size_t count = 0;
    size_t capacity = 8;

    values = malloc(capacity * sizeof(int));
    if (!values) return NULL;

    if (!yaml_parser_parse(parser, &event)) { free(values); return NULL; }
    if (event.type != YAML_SEQUENCE_START_EVENT) {
        yaml_event_delete(&event);
        free(values);
        return NULL;
    }
    yaml_event_delete(&event);

    while (1) {
        if (!yaml_parser_parse(parser, &event)) { free(values); return NULL; }
        if (event.type == YAML_SEQUENCE_END_EVENT) {
            yaml_event_delete(&event);
            break;
        }
        if (event.type == YAML_SCALAR_EVENT) {
            if (count >= capacity) {
                capacity *= 2;
                int *tmp = realloc(values, capacity * sizeof(int));
                if (!tmp) { free(values); yaml_event_delete(&event); return NULL; }
                values = tmp;
            }
            const char *str = (const char *)event.data.scalar.value;
            char *endptr;
            errno = 0;
            long val = strtol(str, &endptr, 10);
            if (errno != 0 || endptr == str || *endptr != '\0') {
                fprintf(stderr, "Invalid integer in config: '%s'\n", str);
                yaml_event_delete(&event);
                continue;
            }
            values[count++] = (int)val;
        }
        yaml_event_delete(&event);
    }

    *out_count = count;
    return values;
}

static char **parse_string_sequence(yaml_parser_t *parser, size_t *out_count) {
    yaml_event_t event;
    char **values = NULL;
    size_t count = 0;
    size_t capacity = 8;

    values = malloc(capacity * sizeof(char *));
    if (!values) return NULL;

    if (!yaml_parser_parse(parser, &event)) { free(values); return NULL; }
    if (event.type != YAML_SEQUENCE_START_EVENT) {
        yaml_event_delete(&event);
        free(values);
        return NULL;
    }
    yaml_event_delete(&event);

    while (1) {
        if (!yaml_parser_parse(parser, &event)) { free(values); return NULL; }
        if (event.type == YAML_SEQUENCE_END_EVENT) {
            yaml_event_delete(&event);
            break;
        }
        if (event.type == YAML_SCALAR_EVENT) {
            if (count >= capacity) {
                capacity *= 2;
                char **tmp = realloc(values, capacity * sizeof(char *));
                if (!tmp) {
                    for (size_t i = 0; i < count; i++) free(values[i]);
                    free(values);
                    yaml_event_delete(&event);
                    return NULL;
                }
                values = tmp;
            }
            values[count] = strdup((const char *)event.data.scalar.value);
            if (!values[count]) {
                for (size_t i = 0; i < count; i++) free(values[i]);
                free(values);
                yaml_event_delete(&event);
                return NULL;
            }
            count++;
        }
        yaml_event_delete(&event);
    }

    *out_count = count;
    return values;
}

/* Skip over an entire YAML mapping or sequence we don't care about. */
static void skip_value(yaml_parser_t *parser) {
    yaml_event_t event;
    if (!yaml_parser_parse(parser, &event)) return;

    switch (event.type) {
    case YAML_SCALAR_EVENT:
        yaml_event_delete(&event);
        return;
    case YAML_SEQUENCE_START_EVENT: {
        yaml_event_delete(&event);
        int depth = 1;
        while (depth > 0) {
            if (!yaml_parser_parse(parser, &event)) return;
            if (event.type == YAML_SEQUENCE_START_EVENT) depth++;
            else if (event.type == YAML_SEQUENCE_END_EVENT) depth--;
            yaml_event_delete(&event);
        }
        return;
    }
    case YAML_MAPPING_START_EVENT: {
        yaml_event_delete(&event);
        int depth = 1;
        while (depth > 0) {
            if (!yaml_parser_parse(parser, &event)) return;
            if (event.type == YAML_MAPPING_START_EVENT) depth++;
            else if (event.type == YAML_MAPPING_END_EVENT) depth--;
            yaml_event_delete(&event);
        }
        return;
    }
    default:
        yaml_event_delete(&event);
        return;
    }
}

/* Parse a data_sizes mapping { small: [...], medium: [...], large: [...] } */
static int parse_data_sizes(yaml_parser_t *parser, struct data_sizes *ds) {
    yaml_event_t event;

    /* consume MAPPING_START */
    if (!yaml_parser_parse(parser, &event)) return -1;
    if (event.type != YAML_MAPPING_START_EVENT) {
        yaml_event_delete(&event);
        return -1;
    }
    yaml_event_delete(&event);

    while (1) {
        if (!yaml_parser_parse(parser, &event)) return -1;
        if (event.type == YAML_MAPPING_END_EVENT) {
            yaml_event_delete(&event);
            break;
        }
        if (event.type == YAML_SCALAR_EVENT) {
            const char *key = (const char *)event.data.scalar.value;
            if (strcmp(key, "small") == 0) {
                yaml_event_delete(&event);
                ds->small = parse_int64_sequence(parser, &ds->small_count);
                if (!ds->small) return -1;
            } else if (strcmp(key, "medium") == 0) {
                yaml_event_delete(&event);
                ds->medium = parse_int64_sequence(parser, &ds->medium_count);
                if (!ds->medium) return -1;
            } else if (strcmp(key, "large") == 0) {
                yaml_event_delete(&event);
                ds->large = parse_int64_sequence(parser, &ds->large_count);
                if (!ds->large) return -1;
            } else {
                yaml_event_delete(&event);
                skip_value(parser);
            }
        } else {
            yaml_event_delete(&event);
        }
    }
    return 0;
}

/* Parse an iterations mapping { warmup: N, measurement: N } */
static int parse_iterations(yaml_parser_t *parser, struct iterations_config *iter) {
    yaml_event_t event;

    if (!yaml_parser_parse(parser, &event)) return -1;
    if (event.type != YAML_MAPPING_START_EVENT) {
        yaml_event_delete(&event);
        return -1;
    }
    yaml_event_delete(&event);

    while (1) {
        if (!yaml_parser_parse(parser, &event)) return -1;
        if (event.type == YAML_MAPPING_END_EVENT) {
            yaml_event_delete(&event);
            break;
        }
        if (event.type == YAML_SCALAR_EVENT) {
            char key[64];
            snprintf(key, sizeof(key), "%s", (const char *)event.data.scalar.value);
            yaml_event_delete(&event);

            if (!yaml_parser_parse(parser, &event)) return -1;
            if (event.type == YAML_SCALAR_EVENT) {
                const char *str = (const char *)event.data.scalar.value;
                char *endptr;
                errno = 0;
                long val = strtol(str, &endptr, 10);
                if (errno == 0 && endptr != str && *endptr == '\0') {
                    if (strcmp(key, "warmup") == 0) iter->warmup = (int)val;
                    else if (strcmp(key, "measurement") == 0) iter->measurement = (int)val;
                } else {
                    fprintf(stderr, "Invalid integer in config: '%s'\n", str);
                }
            }
            yaml_event_delete(&event);
        } else {
            yaml_event_delete(&event);
        }
    }
    return 0;
}

static void free_data_sizes(struct data_sizes *ds) {
    free(ds->small);
    free(ds->medium);
    free(ds->large);
    ds->small = NULL;
    ds->medium = NULL;
    ds->large = NULL;
    ds->small_count = ds->medium_count = ds->large_count = 0;
}

static void free_quick_config(struct quick_config *qc) {
    if (!qc) return;
    free_data_sizes(&qc->data_sizes);
    free(qc->concurrency_levels);
    for (size_t i = 0; i < qc->test_types_count; i++) free(qc->test_types[i]);
    free(qc->test_types);
    free(qc);
}

/* Parse the quick_config mapping */
static struct quick_config *parse_quick_config(yaml_parser_t *parser) {
    yaml_event_t event;
    struct quick_config *qc = calloc(1, sizeof(struct quick_config));
    if (!qc) return NULL;

    if (!yaml_parser_parse(parser, &event)) { free(qc); return NULL; }
    if (event.type != YAML_MAPPING_START_EVENT) {
        yaml_event_delete(&event);
        free(qc);
        return NULL;
    }
    yaml_event_delete(&event);

    while (1) {
        if (!yaml_parser_parse(parser, &event)) { free_quick_config(qc); return NULL; }
        if (event.type == YAML_MAPPING_END_EVENT) {
            yaml_event_delete(&event);
            break;
        }
        if (event.type == YAML_SCALAR_EVENT) {
            const char *key = (const char *)event.data.scalar.value;
            if (strcmp(key, "data_sizes") == 0) {
                yaml_event_delete(&event);
                if (parse_data_sizes(parser, &qc->data_sizes) != 0) { free_quick_config(qc); return NULL; }
            } else if (strcmp(key, "iterations") == 0) {
                yaml_event_delete(&event);
                if (parse_iterations(parser, &qc->iterations) != 0) { free_quick_config(qc); return NULL; }
            } else if (strcmp(key, "concurrency_levels") == 0) {
                yaml_event_delete(&event);
                qc->concurrency_levels = parse_int_sequence(parser, &qc->concurrency_count);
                if (!qc->concurrency_levels) { free_quick_config(qc); return NULL; }
            } else if (strcmp(key, "test_types") == 0) {
                yaml_event_delete(&event);
                qc->test_types = parse_string_sequence(parser, &qc->test_types_count);
                if (!qc->test_types) { free_quick_config(qc); return NULL; }
            } else {
                yaml_event_delete(&event);
                skip_value(parser);
            }
        } else {
            yaml_event_delete(&event);
        }
    }
    return qc;
}

/* ---- public API --------------------------------------------------------- */

struct test_config *load_config(const char *path) {
    FILE *fp = fopen(path, "r");
    if (!fp) {
        fprintf(stderr, "Config file not found: %s\n", path);
        return NULL;
    }

    yaml_parser_t parser;
    if (!yaml_parser_initialize(&parser)) {
        fclose(fp);
        return NULL;
    }
    yaml_parser_set_input_file(&parser, fp);

    struct test_config *config = calloc(1, sizeof(struct test_config));
    if (!config) {
        yaml_parser_delete(&parser);
        fclose(fp);
        return NULL;
    }

    /* defaults */
    config->iterations.warmup = 5;
    config->iterations.measurement = 10;

    yaml_event_t event;

    /* consume STREAM_START and DOCUMENT_START */
    yaml_parser_parse(&parser, &event); yaml_event_delete(&event);
    yaml_parser_parse(&parser, &event); yaml_event_delete(&event);

    /* top-level MAPPING_START */
    yaml_parser_parse(&parser, &event);
    if (event.type != YAML_MAPPING_START_EVENT) {
        yaml_event_delete(&event);
        yaml_parser_delete(&parser);
        fclose(fp);
        free_config(config);
        return NULL;
    }
    yaml_event_delete(&event);

    while (1) {
        if (!yaml_parser_parse(&parser, &event)) break;
        if (event.type == YAML_MAPPING_END_EVENT) {
            yaml_event_delete(&event);
            break;
        }
        if (event.type == YAML_SCALAR_EVENT) {
            const char *key = (const char *)event.data.scalar.value;
            if (strcmp(key, "data_sizes") == 0) {
                yaml_event_delete(&event);
                parse_data_sizes(&parser, &config->data_sizes);
            } else if (strcmp(key, "iterations") == 0) {
                yaml_event_delete(&event);
                parse_iterations(&parser, &config->iterations);
            } else if (strcmp(key, "concurrency_levels") == 0) {
                yaml_event_delete(&event);
                config->concurrency_levels = parse_int_sequence(&parser, &config->concurrency_count);
            } else if (strcmp(key, "quick_config") == 0) {
                yaml_event_delete(&event);
                config->quick_config = parse_quick_config(&parser);
            } else {
                yaml_event_delete(&event);
                skip_value(&parser);
            }
        } else {
            yaml_event_delete(&event);
        }
    }

    yaml_parser_delete(&parser);
    fclose(fp);

    if (config->iterations.measurement <= 0) {
        fprintf(stderr, "Invalid config: iterations.measurement must be > 0\n");
        free_config(config);
        return NULL;
    }

    return config;
}

void adjust_for_quick_test(struct test_config *config) {
    if (!config || !config->quick_config) return;

    struct quick_config *qc = config->quick_config;

    /* Override data sizes: use quick small, clear medium and large */
    free(config->data_sizes.small);
    config->data_sizes.small = NULL;
    config->data_sizes.small_count = 0;

    if (qc->data_sizes.small_count > 0) {
        config->data_sizes.small = malloc(qc->data_sizes.small_count * sizeof(int64_t));
        if (config->data_sizes.small) {
            memcpy(config->data_sizes.small, qc->data_sizes.small,
                   qc->data_sizes.small_count * sizeof(int64_t));
            config->data_sizes.small_count = qc->data_sizes.small_count;
        }
    }

    free(config->data_sizes.medium);
    config->data_sizes.medium = NULL;
    config->data_sizes.medium_count = 0;

    free(config->data_sizes.large);
    config->data_sizes.large = NULL;
    config->data_sizes.large_count = 0;

    /* Override iterations */
    config->iterations.warmup = qc->iterations.warmup;
    config->iterations.measurement = qc->iterations.measurement;

    /* Override concurrency levels */
    if (qc->concurrency_count > 0) {
        free(config->concurrency_levels);
        config->concurrency_levels = malloc(qc->concurrency_count * sizeof(int));
        if (config->concurrency_levels) {
            memcpy(config->concurrency_levels, qc->concurrency_levels,
                   qc->concurrency_count * sizeof(int));
            config->concurrency_count = qc->concurrency_count;
        }
    }
}

int should_run_test_type(const struct test_config *config, const char *type) {
    if (!config || !config->quick_config) return 1;
    if (config->quick_config->test_types_count == 0) return 1;

    for (size_t i = 0; i < config->quick_config->test_types_count; i++) {
        if (strcmp(config->quick_config->test_types[i], type) == 0) {
            return 1;
        }
    }
    return 0;
}

void get_all_data_sizes(const struct test_config *config, int64_t **out_sizes, size_t *out_count) {
    size_t total = config->data_sizes.small_count
                 + config->data_sizes.medium_count
                 + config->data_sizes.large_count;

    if (total == 0) {
        *out_sizes = NULL;
        *out_count = 0;
        return;
    }

    int64_t *sizes = malloc(total * sizeof(int64_t));
    if (!sizes) {
        *out_sizes = NULL;
        *out_count = 0;
        return;
    }

    size_t idx = 0;
    for (size_t i = 0; i < config->data_sizes.small_count; i++)
        sizes[idx++] = config->data_sizes.small[i];
    for (size_t i = 0; i < config->data_sizes.medium_count; i++)
        sizes[idx++] = config->data_sizes.medium[i];
    for (size_t i = 0; i < config->data_sizes.large_count; i++)
        sizes[idx++] = config->data_sizes.large[i];

    *out_sizes = sizes;
    *out_count = total;
}


void free_config(struct test_config *config) {
    if (!config) return;

    free_data_sizes(&config->data_sizes);
    free(config->concurrency_levels);

    if (config->quick_config) {
        free_quick_config(config->quick_config);
    }

    free(config);
}
