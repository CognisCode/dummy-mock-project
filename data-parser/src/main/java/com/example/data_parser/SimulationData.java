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

    private int value_score;
    private int high_score;
    private int close_score;
    private int custom_score;
    private Float value_x;
    private Float value_y;
    private Float high_x;
    private Float high_y;
    private Float close_x;
    private Float close_y;
    private Float custom_x;
    private Float custom_y;
    private Instant timestamp;

    public Long getId() {
        return id;
    }

    public void setId(Long id) {
        this.id = id;
    }

    public int getValueScore() {
        return value_score;
    }

    public void setValueScore(int value_score) {
        this.value_score = value_score;
    }

    public int getHighScore() {
        return high_score;
    }

    public void setHighScore(int high_score) {
        this.high_score = high_score;
    }

    public int getCloseScore() {
        return close_score;
    }

    public void setCloseScore(int close_score) {
        this.close_score = close_score;
    }

    public int getCustomScore() {
        return custom_score;
    }

    public void setCustomScore(int custom_score) {
        this.custom_score = custom_score;
    }

    public Instant getTimestamp() {
        return timestamp;
    }

    public void setTimestamp(Instant timestamp) {
        this.timestamp = timestamp;
    }

    public Float getValueX() {
        return value_x;
    }

    public void setValueX(Float value_x) {
        this.value_x = value_x;
    }

    public Float getValueY() {
        return value_y;
    }

    public void setValueY(Float value_y) {
        this.value_y = value_y;
    }

    public Float getHighX() {
        return high_x;
    }

    public void setHighX(Float high_x) {
        this.high_x = high_x;
    }

    public Float getHighY() {
        return high_y;
    }

    public void setHighY(Float high_y) {
        this.high_y = high_y;
    }

    public Float getCloseX() {
        return close_x;
    }

    public void setCloseX(Float close_x) {
        this.close_x = close_x;
    }

    public Float getCloseY() {
        return close_y;
    }

    public void setCloseY(Float close_y) {
        this.close_y = close_y;
    }

    public Float getCustomX() {
        return custom_x;
    }

    public void setCustomX(Float custom_x) {
        this.custom_x = custom_x;
    }

    public Float getCustomY() {
        return custom_y;
    }

    public void setCustomY(Float custom_y) {
        this.custom_y = custom_y;
    }
}
