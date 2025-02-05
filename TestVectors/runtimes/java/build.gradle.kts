tasks.wrapper {
    gradleVersion = "7.6"
}

plugins {
    `java-library`
    `maven-publish`
    `application`
}

var props = Properties().apply {
    load(FileInputStream(File(rootProject.rootDir, "../../project.properties")))
}
var mplVersion = props.getProperty("mplDependencyJavaVersion")

group = "software.amazon.cryptography"
version = "1.0.0-SNAPSHOT"
description = "AwsEncryptionSDKJavaTestVectors"

java {
    toolchain.languageVersion.set(JavaLanguageVersion.of(8))
    sourceSets["main"].java {
        srcDir("src/main/java")
        srcDir("src/main/dafny-generated")
        srcDir("src/main/smithy-generated")
    }
    sourceSets["test"].java {
        srcDir("src/test/dafny-generated")
        srcDir("src/test/java")
    }
}

repositories {
    // Use Maven Central for resolving dependencies.
    mavenCentral()
    mavenLocal()
}
dependencies {
    implementation("org.dafny:DafnyRuntime:4.8.0")
    implementation("software.amazon.smithy.dafny:conversion:0.1")
    implementation("software.amazon.cryptography:aws-cryptographic-material-providers:${mplVersion}")
    implementation("software.amazon.cryptography:TestAwsCryptographicMaterialProviders:${mplVersion}")
    implementation("software.amazon.cryptography:aws-encryption-sdk:1.0.0-SNAPSHOT")
    implementation("com.amazonaws:aws-encryption-sdk-java:3.0.1")
    implementation(platform("software.amazon.awssdk:bom:2.25.1"))
    implementation("software.amazon.awssdk:dynamodb")
    implementation("software.amazon.awssdk:dynamodb-enhanced")
    implementation("software.amazon.awssdk:kms")
}

tasks.register<JavaExec>("runTests") {
    dependsOn("copyKeysJSON")
    mainClass.set("TestsFromDafny")
    classpath = sourceSets["test"].runtimeClasspath
}

tasks.register<Copy>("copyKeysJSON") {
    from(layout.projectDirectory.file("../../dafny/TestVectors/test/keys.json"))
    into(layout.projectDirectory.dir("dafny/TestVectors/test"))
}

tasks.register<Copy>("copyKeysJSONCurr") {
    from(layout.projectDirectory.file("../../dafny/TestVectors/test/keys.json"))
    into(layout.projectDirectory.dir("."))
}

application {
    mainClass.set("ImplementationFromDafny")
}
