// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
package software.amazon.cryptography.encryptionsdk.model;

public class OpaqueWithTextError extends RuntimeException {

  /**
   * The unexpected object encountered. It MIGHT BE an Exception, but that is not guaranteed.
   */
  private final Object obj;

  /**
   * The text equivalent of obj.
   */
  private final String objMessage;

  protected OpaqueWithTextError(BuilderImpl builder) {
    super(messageFromBuilder(builder), builder.cause());
    this.obj = builder.obj();
    this.objMessage = builder.objMessage();
  }

  private static String messageFromBuilder(Builder builder) {
    if (builder.message() != null) {
      return builder.message();
    }
    if (builder.cause() != null) {
      return builder.cause().getMessage();
    }
    return null;
  }

  /**
   * See {@link Throwable#getMessage()}.
   */
  public String message() {
    return this.getMessage();
  }

  /**
   * See {@link Throwable#getCause()}.
   */
  public Throwable cause() {
    return this.getCause();
  }

  /**
   * @return The unexpected object encountered. It MIGHT BE an Exception, but that is not guaranteed.
   */
  public Object obj() {
    return this.obj;
  }

  /**
   * @return The text equivalent of obj.
   */
  public String objMessage() {
    return this.objMessage;
  }

  public Builder toBuilder() {
    return new BuilderImpl(this);
  }

  public static Builder builder() {
    return new BuilderImpl();
  }

  public interface Builder {
    /**
     * @param message The detailed message. The detail message is saved for later retrieval by the {@link #getMessage()} method.
     */
    Builder message(String message);

    /**
     * @return The detailed message. The detail message is saved for later retrieval by the {@link #getMessage()} method.
     */
    String message();

    /**
     * @param cause The cause (which is saved for later retrieval by the {@link #getCause()} method). (A {@code null} value is permitted, and indicates that the cause is nonexistent or unknown.)
     */
    Builder cause(Throwable cause);

    /**
     * @return The cause (which is saved for later retrieval by the {@link #getCause()} method). (A {@code null} value is permitted, and indicates that the cause is nonexistent or unknown.)
     */
    Throwable cause();

    /**
     * @param obj The unexpected object encountered. It MIGHT BE an Exception, but that is not guaranteed.
     */
    Builder obj(Object obj);

    /**
     * @return The unexpected object encountered. It MIGHT BE an Exception, but that is not guaranteed.
     */
    Object obj();

    /**
     * @param objMessage The text equivalent of obj.
     */
    Builder objMessage(String objMessage);

    /**
     * @return The text equivalent of obj.
     */
    String objMessage();

    OpaqueWithTextError build();
  }

  static class BuilderImpl implements Builder {

    protected String message;

    protected Throwable cause;

    protected Object obj;

    protected String objMessage;

    protected BuilderImpl() {}

    protected BuilderImpl(OpaqueWithTextError model) {
      this.cause = model.getCause();
      this.message = model.getMessage();
      this.obj = model.obj();
      this.objMessage = model.objMessage();
    }

    public Builder message(String message) {
      this.message = message;
      return this;
    }

    public String message() {
      return this.message;
    }

    public Builder cause(Throwable cause) {
      this.cause = cause;
      return this;
    }

    public Throwable cause() {
      return this.cause;
    }

    public Builder obj(Object obj) {
      this.obj = obj;
      return this;
    }

    public Object obj() {
      return this.obj;
    }

    public Builder objMessage(String objMessage) {
      this.objMessage = objMessage;
      return this;
    }

    public String objMessage() {
      return this.objMessage;
    }

    public OpaqueWithTextError build() {
      if (
        this.obj != null && this.cause == null && this.obj instanceof Throwable
      ) {
        this.cause = (Throwable) this.obj;
      } else if (this.obj == null && this.cause != null) {
        this.obj = this.cause;
      }
      return new OpaqueWithTextError(this);
    }
  }
}
