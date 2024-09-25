package com.example.data_parser;

import org.springframework.data.jpa.repository.JpaRepository;
import org.springframework.stereotype.Repository;

@Repository  // Ensure this annotation is present
public interface SimulationDataRepository extends JpaRepository<SimulationData, Long> {}