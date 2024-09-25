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

@Service
public class OSCReceiverService {

    private static final int PORT = 9007; // Define the port where the OSC messages will be received

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

    @Transactional
    private void handleOSCMessage(OSCMessageEvent event) {
        OSCMessage message = event.getMessage();
        System.out.println("Received OSC Message: " + message.getAddress());

        Float smartscore = (Float) message.getArguments().get(0);
        Float highScore = (Float) message.getArguments().get(1);
        Float closeScore = (Float) message.getArguments().get(2);

        SimulationData simulationData = new SimulationData();
        simulationData.setSmartscore(smartscore);
        simulationData.setHighScore(highScore);
        simulationData.setCloseScore(closeScore);
        simulationData.setTimestamp(Instant.now());

        // Save the oscData to the database
        simulationDataRepository.save(simulationData);
    }
}