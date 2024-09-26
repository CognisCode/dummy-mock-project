package com.example.data_parser;

import org.springframework.boot.SpringApplication;
import org.springframework.boot.autoconfigure.SpringBootApplication;

// Spring application made using boilerplate to setup a connection to my rust simulation and parse the osc data into
// a postgres database. This mimics a microservice for demonstration purposes yet no businness logic is implemented
// on the parsed data
// no logging or proper error handling has been implemented
@SpringBootApplication(scanBasePackages = {"com.example.data_parser", "com.example.data_parser.repository"})public class DataParserApplication {
	public static void main(String[] args) {
		SpringApplication.run(DataParserApplication.class, args);
	}
}
