FROM openjdk:17-jdk-slim

WORKDIR /app

COPY target/data-parser-3.3.4.jar app.jar

EXPOSE 9007

CMD ["java", "-jar", "app.jar"]
