package com.example.data_parser;

import com.illposed.osc.messageselector.OSCPatternAddressMessageSelector;
import com.illposed.osc.transport.OSCPortIn;
import com.illposed.osc.*;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.stereotype.Service;
import org.springframework.transaction.annotation.Transactional;

import javax.annotation.PostConstruct;
import java.io.IOException;
import java.time.Instant;
import java.util.List;

@Service
public class OSCReceiverService {

    private static final int PORT = 9007;

    @Autowired
    private SimulationDataRepository simulationDataRepository;

    @PostConstruct
    public void startListening() {
        try {
            OSCPortIn receiver = new OSCPortIn(PORT);
            OSCMessageListener listener = new OSCMessageListener() {
                @Override
                public void acceptMessage(OSCMessageEvent event) {
                    handleOSCMessage(event);
                }
            };

            OSCPatternAddressMessageSelector selector = new OSCPatternAddressMessageSelector("/simulation/scores");
            receiver.getDispatcher().addListener(selector, listener);
            receiver.startListening();
            System.out.println("Started OSC Receiver on port " + PORT);
        } catch (IOException e) {
            e.printStackTrace();
        }
    }

    // This function would be the entry to apply business logic to the data
    @Transactional
    private void handleOSCMessage(OSCMessageEvent event) {
        OSCMessage message = event.getMessage();

//        for (Object argument : message.getArguments()) {
//            System.out.println("Argument: " + argument);
//        }
//
//        System.out.println("Received OSC Message: " + message.getAddress());

        int smart_score = (int) message.getArguments().get(0);
        int high_score = (int) message.getArguments().get(1);
        int close_score = (int) message.getArguments().get(2);
        int genetic_score = (int) message.getArguments().get(3);
        Float smart_start_x =  (Float) message.getArguments().get(4);
        Float smart_start_y =  (Float) message.getArguments().get(5);
        Float high_start_x =  (Float) message.getArguments().get(6);
        Float high_start_y =  (Float) message.getArguments().get(7);
        Float close_start_x =  (Float) message.getArguments().get(8);
        Float close_start_y =  (Float) message.getArguments().get(9);
        Float genetic_start_x =  (Float) message.getArguments().get(10);
        Float genetic_start_y =  (Float) message.getArguments().get(11);

        SimulationData simulationData = new SimulationData();

        simulationData.setSmartScore(smart_score);
        simulationData.setHighScore(high_score);
        simulationData.setCloseScore(close_score);
        simulationData.setGeneticScore(genetic_score);
        simulationData.setSmartStartX(smart_start_x);
        simulationData.setSmartStartY(smart_start_y);
        simulationData.setHighStartX(high_start_x);
        simulationData.setHighStartY(high_start_y);
        simulationData.setCloseStartX(close_start_x);
        simulationData.setCloseStartY(close_start_y);
        simulationData.setGeneticStartX(genetic_start_x);
        simulationData.setGeneticStartY(genetic_start_y);

        simulationData.setTimestamp(Instant.now());

        simulationDataRepository.save(simulationData);
    }
}