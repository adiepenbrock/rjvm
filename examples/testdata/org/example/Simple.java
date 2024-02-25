package org.example;

public class Simple implements Runnable {

    public @interface JsonSerializable {
        public String key() default "";
    }

    private String firstname;
    private int age;

    public static void main(String[] args) {
        System.out.println("Hello World");
    }

    public void run() {
        System.out.println("Hello World");
    }

    @JsonSerializable(key = "name")
    public String something(String input) {
        return input;
    }
}
