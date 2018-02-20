package com.oztz.hackinglabmobile.businessclasses;

/**
 * Created by Tobi on 25.03.2015.
 */
public class Social {
    public int socialID;
    public String text;
    public String status;
    public String media;
    public String authorName;
    public int userIDFK;
    public int mediaIDFK;
    public int eventIDFK;

    public Social(String text, String status, String media, String authorName, int userIDFK, int mediaIDFK, int eventIDFK){
        this.text = text;
        this.status = status;
        this.authorName = authorName;
        this.userIDFK = userIDFK;
        this.mediaIDFK = mediaIDFK;
        this.eventIDFK = eventIDFK;
        this.media = media;
    }
}
