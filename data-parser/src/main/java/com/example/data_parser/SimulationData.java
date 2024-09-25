package com.example.data_parser;

import jakarta.persistence.Entity;
import jakarta.persistence.GeneratedValue;
import jakarta.persistence.GenerationType;
import jakarta.persistence.Id;
import java.time.Instant;

@Entity
public class SimulationData {

    @Id
    @GeneratedValue(strategy = GenerationType.IDENTITY)
    private Long id;

    private Float smartscore;
    private Float highScore;
    private Float closeScore;
    private Instant timestamp;

    public Long getId() {
        return id;
    }

    public void setId(Long id) {
        this.id = id;
    }

    public Float getSmartscore() {
        return smartscore;
    }

    public void setSmartscore(Float smartscore) {
        this.smartscore = smartscore;
    }

    public Float getHighScore() {
        return highScore;
    }

    public void setHighScore(Float highScore) {
        this.highScore = highScore;
    }

    public Float getCloseScore() {
        return closeScore;
    }

    public void setCloseScore(Float closeScore) {
        this.closeScore = closeScore;
    }

    public Instant getTimestamp() {
        return timestamp;
    }

    public void setTimestamp(Instant timestamp) {
        this.timestamp = timestamp;
    }
}
