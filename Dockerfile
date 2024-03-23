# Use the official Ubuntu base image
FROM ubuntu:latest

# Update the package repository and install Java
RUN apt-get update && \
    apt-get install -y openjdk-11-jdk
RUN apt-get install -y tmux
# Set the JAVA_HOME environment variable
ENV JAVA_HOME /usr/lib/jvm/java-11-openjdk-amd64

# (Optional) Set the working directory
WORKDIR /app

# (Optional) Copy your Java application into the container
# COPY ./your-java-app.jar /app/your-java-app.jar

# (Optional) Command to run your Java application
# CMD ["java", "-jar", "your-java-app.jar"]
COPY data.db /app/
COPY Rocket.toml /app/Rocket.toml
COPY static /app/static
COPY templates /app/templates
COPY target/release/minecraft-dashboard /app/minecraft-dashboard
COPY minecraftdata /app/minecraftdata
# Expose any ports your application needs
CMD ["/app/minecraft-dashboard"]

EXPOSE 8080

# You can customize this Dockerfile further according to your specific requirements.
