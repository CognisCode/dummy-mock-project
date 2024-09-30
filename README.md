![](https://github.com/YassinYassinYassin/yassin-mock-project/blob/master/resources/mock-project-3.gif)


# Mock project

**Description:**  
This mock project simulates a complex setting (data generation with non-trivial behaviour) using a simulation written in Rust which processes and stores the data via a Java Spring Boot application that writes to a PostgreSQL database, and then performs data analysis with a Python application. (no logging, proper error handling or testing has been implemented yet)

## Table of Contents
1. [Project Structure](#project-structure)
2. [Project overview](#setup-instructions)
   - [1. Rust Simulation](#1-rust-simulation)
   - [2. Java Spring Boot App](#2-java-spring-boot-app)
   - [3. PostgreSQL Database](#3-postgresql-database)
   - [4. Python Data Analysis](#4-python-data-analysis)
3. [Usage](#usage)



## 1. Project Structure
- data-analysis: Python script to manually analyse data.
- data-parser: source code for Java spring app that acts as a microservice to process and store data.
- initdb: init scripts for the postgres database
- simulation: Rust application for simulating competitive strategies in resource gathering.



## 2. Code Instructions

### 2.1 Rust Simulation
This program uses nannou to setup a visual simulation of a resource gathering game. Three chasers _smart_, _close_, _high_ and _genetic_ are generated. The goal is to accumelate as much resources as possible. Their are high resources labeled green worth 200 and low rewards labeled blue worth 50. 
- close: chases the closest resource regardless of value.
- high: chases high resources first. Only goes for low resources as all the high resources have been depleted.
- smart: Chases the reward with the highest value/distance ratio. 
- genetic: Uses a simple genetic algorithm to find the best value using the reward value, distance to reward and the distance of other players to the reward. Optimiziation happens through random mutations.  

The program sends the scores and location of each chaser through OSC data for the java app the process. 

### 2.2 Java Spring Boot App
This program mimics a microservice that sits between the simulation and database storage. A Dockerfile has been provided to make deployment through docker compose possible. In an enterprise setting all the data parsing or business logic before storing would be added here.

### 2.3 PostgreSQL Database
This database is deployed locally using docker. The initdb folder initializes the database with the right tables and prevents the database exceeding 5000 rows. 

### 2.4 Python data-analysis
Single script that live analyses the streaming data. Generates plots to analyse the scores and total distance between a chaser and other chasers. Intricate data analysis would be added here.    

## 3 Usage
- 1: from project root:  `docker compose up -d`
- 2: from data-parser folder:  `mvn spring-boot:run`
- 3: from  simulation folder: `cargo run`
- 4: from data-analysis folder:  step 1: `python3 -m venv .` 
step 2:`source bin/activate` step3: `pip install -r requirements.txt`
step 4: `python3 src/analysis.py`

or 
- 1: from data-parser folder: `docker build -t data-parser .` 
- 2: from project root uncomment data-parser in the docker-compose.yml
- 3: from project root:  `docker compose up -d`
- 4: Continue with step 3 above.`

note: Ideally if the entire project runs from the same machine a single docker compose is the best suited option. The streaming plot from the python script needs to be posted to a webapp through an API since docker runs headlessly. The nannou simulation on the other hand needs to be mounted against a virtual display. 