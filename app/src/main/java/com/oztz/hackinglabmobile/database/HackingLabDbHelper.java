package com.oztz.hackinglabmobile.database;

import android.content.Context;
import android.database.sqlite.SQLiteDatabase;
import android.database.sqlite.SQLiteOpenHelper;

/**
 * Created by Tobi on 08.05.2015.
 */
public class HackingLabDbHelper extends SQLiteOpenHelper {
    public static final int DATABASE_VERSION = 4;
    public static final String DATABASE_NAME = "HackingLabMobile.db";

    //TableNames
    public static final String QR_TABLE_NAME = "qrcodes";
    public static final String CONTENT_TABLE_NAME = "contents";

    //DropTables
    public static final String DROP_QR_TABLE = "DROP TABLE IF EXISTS " + QR_TABLE_NAME;
    public static final String DROP_CONTENT_TABLE = "DROP TABLE IF EXISTS " + CONTENT_TABLE_NAME;

    //CreateTablse
    public static final String CREATE_QR_TABLE = "CREATE TABLE " + QR_TABLE_NAME + " (" +
            "_id INTEGER PRIMARY KEY AUTOINCREMENT," +
            "role TEXT," +
            "eventid INTEGER," +
            "secret TEXT" +
            " )";
    public static final String CREATE_CONTENT_TABLE = "CREATE TABLE " + CONTENT_TABLE_NAME + " (" +
            "_id INTEGER PRIMARY KEY AUTOINCREMENT," +
            "key TEXT," +
            "content TEXT," +
            "lastupdate INTEGER" +
            " )";



    public HackingLabDbHelper(Context context) {
        super(context, DATABASE_NAME, null, DATABASE_VERSION);
    }

    @Override
    public void onCreate(SQLiteDatabase db) {
        db.execSQL(CREATE_QR_TABLE);
        db.execSQL(CREATE_CONTENT_TABLE);
    }

    @Override
    public void onUpgrade(SQLiteDatabase db, int oldVersion, int newVersion) {
        db.execSQL(DROP_QR_TABLE);
        db.execSQL(DROP_CONTENT_TABLE);
        onCreate(db);
    }
}
