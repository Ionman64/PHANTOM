package com.oztz.hackinglabmobile.businessclasses;

/**
 * Created by Tobi on 08.05.2015.
 */
public class Slider {
    public int sliderID;
    public int votingIDFK;
    public int weigth;
    public String name;

    public Slider(int sliderID, int votingIDFK, int weigth, String name){
        this.sliderID = sliderID;
        this.votingIDFK = votingIDFK;
        this.weigth = weigth;
        this.name = name;
    }
}
