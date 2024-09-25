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

    private Float smart_score;
    private Float high_score;
    private Float close_score;

    private Float smart_start_x;
    private Float smart_start_y;

    private Float high_start_x;
    private Float high_start_y;

    private Float close_start_x;
    private Float close_start_y;

    private Instant timestamp;

    public Long getId() {
        return id;
    }

    public void setId(Long id) {
        this.id = id;
    }

    public Float getSmartScore() {
        return smart_score;
    }

    public void setSmartScore(Float smart_score) {
        this.smart_score = smart_score;
    }

    public Float getHighScore() {
        return high_score;
    }

    public void setHighScore(Float high_score) {
        this.high_score = high_score;
    }

    public Float getCloseScore() {
        return close_score;
    }

    public void setCloseScore(Float close_score) {
        this.close_score = close_score;
    }

    public Instant getTimestamp() {
        return timestamp;
    }

    public void setTimestamp(Instant timestamp) {
        this.timestamp = timestamp;
    }

    public Float getSmartStartX() {
        return smart_start_x;
    }

    public void setSmartStartX(Float smart_start_x) {
        this.smart_start_x = smart_start_x;
    }

    public Float getSmartStartY() {
        return smart_start_y;
    }

    public void setSmartStartY(Float smart_start_y) {
        this.smart_start_y = smart_start_y;
    }

    public Float getHighStartX() {
        return high_start_x;
    }

    public void setHighStartX(Float high_start_x) {
        this.high_start_x = high_start_x;
    }

    public Float getHighStartY() {
        return high_start_y;
    }

    public void setHighStartY(Float high_start_y) {
        this.high_start_y = high_start_y;
    }

    public Float getCloseStartX() {
        return close_start_x;
    }

    public void setCloseStartX(Float close_start_x) {
        this.close_start_x = close_start_x;
    }

    public Float getCloseStartY() {
        return close_start_y;
    }

    public void setCloseStartY(Float close_start_y) {
        this.close_start_y = close_start_y;
    }
}
