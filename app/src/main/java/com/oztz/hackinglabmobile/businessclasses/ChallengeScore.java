package com.oztz.hackinglabmobile.businessclasses;

/**
 * Created by Tobi on 23.05.2015.
 */
public class ChallengeScore {
    public int ChallengeID;
    public String ChallengeName;
    public int GroupID;
    public String GroupName;
    public int Score;
    public int ChallengeLevel;

    public ChallengeScore(int ChallengeID, String ChallengeName, int GroupID, String GroupName, int Score, int ChallengeLevel){
        this.ChallengeID = ChallengeID;
        this.ChallengeName = ChallengeName;
        this.GroupID = GroupID;
        this.GroupName = GroupName;
        this.Score = Score;
        this.ChallengeLevel = ChallengeLevel;
    }
}
