package com.oztz.hackinglabmobile.businessclasses;

/**
 * Created by Tobi on 02.04.2015.
 */
public class Voting {
    public int eventIDFK;
    public int juryCount;
    public int presentationIDFK;
    public int round;
    public int sliderMaxValue;
    public int votingID;
    public String votingStarted;
    public String votingDuration;
    public String arithmeticMode;
    public String name;
    public String status;


    public String votingEnd;
    public long timeUntilEnd;


    public long getTimeDiff(String serverTime){
        String[] startParts = serverTime.split(":");
        String[] endParts = this.votingEnd.split(":");
        if(startParts.length == 3 && endParts.length == 3) {
            long startMillis = Integer.parseInt(startParts[0]) * 3600000 +
                    Integer.parseInt(startParts[1]) * 60000 +
                    Integer.parseInt(startParts[2]) * 1000;
            long endMillis = Integer.parseInt(endParts[0]) * 3600000 +
                    Integer.parseInt(endParts[1]) * 60000 +
                    Integer.parseInt(endParts[2]) * 1000;
            return endMillis - startMillis;
        }
        return 0;
    }
}
