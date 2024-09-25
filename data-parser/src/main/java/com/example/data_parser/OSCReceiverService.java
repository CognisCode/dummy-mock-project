package com.example.data_parser;

import com.illposed.osc.messageselector.OSCPatternAddressMessageSelector;
import com.illposed.osc.transport.OSCPortIn;
import com.illposed.osc.*;
import org.springframework.stereotype.Service;

import javax.annotation.PostConstruct;
import java.io.IOException;
import java.net.SocketException;

@Service
public class OSCReceiverService {

    private static final int PORT = 9007; // Define the port where the OSC messages will be received

    @PostConstruct
    public void startListening() {
        try {
            // Create an OSC receiver on the specified port
            OSCPortIn receiver = new OSCPortIn(PORT);

            // Define an OSC message listener using OSCMessageEvent
            OSCMessageListener listener = new OSCMessageListener() {
                @Override
                public void acceptMessage(OSCMessageEvent event) {
                    handleOSCMessage(event);
                }
            };

            // Use OSCPatternAddressMessageSelector to listen for a specific OSC address
            OSCPatternAddressMessageSelector selector = new OSCPatternAddressMessageSelector("/simulation/scores");

            // Add the listener with the selector to the receiver's dispatcher
            receiver.getDispatcher().addListener(selector, listener);

            // Start listening for incoming OSC messages
            receiver.startListening();
            System.out.println("Started OSC Receiver on port " + PORT);
        } catch (IOException e) {
            e.printStackTrace();
        }
    }

    private void handleOSCMessage(OSCMessageEvent event) {
        OSCMessage message = event.getMessage(); // Get the actual OSCMessage
        System.out.println("Received OSC Message: " + message.getAddress());
        for (Object argument : message.getArguments()) {
            System.out.println("Argument: " + argument);
        }
    }
}