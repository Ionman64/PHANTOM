package com.oztz.hackinglabmobile.businessclasses;

/**
 * Created by Tobi on 02.04.2015.
 */
public class NavigationItem {
    public static final int TYPE_TITLE = 0;
    public static final int TYPE_ITEM = 1;

    public String text;
    public int type;

    public NavigationItem(String text, int type){
        this.text = text;
        this.type = type;
    }
}
