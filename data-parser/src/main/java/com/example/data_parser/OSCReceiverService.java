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

        int value_score = (int) message.getArguments().get(0);
        int high_score = (int) message.getArguments().get(1);
        int close_score = (int) message.getArguments().get(2);
        int custom_score = (int) message.getArguments().get(3);
        Float value_x =  (Float) message.getArguments().get(4);
        Float value_y =  (Float) message.getArguments().get(5);
        Float high_x =  (Float) message.getArguments().get(6);
        Float high_y =  (Float) message.getArguments().get(7);
        Float close_x =  (Float) message.getArguments().get(8);
        Float close_y =  (Float) message.getArguments().get(9);
        Float custom_x =  (Float) message.getArguments().get(10);
        Float custom_y =  (Float) message.getArguments().get(11);

        SimulationData simulationData = new SimulationData();

        simulationData.setValueScore(value_score);
        simulationData.setHighScore(high_score);
        simulationData.setCloseScore(close_score);
        simulationData.setCustomScore(custom_score);
        simulationData.setValueX(value_x);
        simulationData.setValueY(value_y);
        simulationData.setHighX(high_x);
        simulationData.setHighY(high_y);
        simulationData.setCloseX(close_x);
        simulationData.setCloseY(close_y);
        simulationData.setCustomX(custom_x);
        simulationData.setCustomY(custom_y);

        simulationData.setTimestamp(Instant.now());

        simulationDataRepository.save(simulationData);
    }
}