package com.oztz.hackinglabmobile.businessclasses;

/**
 * Created by Tobi on 08.05.2015.
 */
public class Vote {
    public int voteID;
    public int score;
    public boolean isJury;
    public int sliderIDFK;
    public int userIDFK;

    public Vote(int voteID, int score, boolean isJury, int sliderIDFK, int userIDFK){
        this.voteID = voteID;
        this.score = score;
        this.isJury = isJury;
        this.sliderIDFK = sliderIDFK;
        this.userIDFK = userIDFK;
    }
}
