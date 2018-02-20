package com.oztz.hackinglabmobile;

import android.content.Intent;
import android.os.Bundle;
import android.support.v4.app.Fragment;
import android.support.v4.app.FragmentManager;
import android.support.v4.widget.DrawerLayout;
import android.support.v7.app.ActionBar;
import android.support.v7.app.ActionBarActivity;
import android.util.Log;
import android.view.Menu;
import android.view.MenuItem;
import android.widget.Toast;

import com.google.zxing.integration.android.IntentIntegrator;
import com.google.zxing.integration.android.IntentResult;
import com.oztz.hackinglabmobile.database.DbOperator;
import com.oztz.hackinglabmobile.fragment.AgendaTabHolderFragment;
import com.oztz.hackinglabmobile.fragment.ChallengesFragment;
import com.oztz.hackinglabmobile.fragment.ConferenceFragment;
import com.oztz.hackinglabmobile.fragment.MainFragment;
import com.oztz.hackinglabmobile.fragment.ScoringFragment;
import com.oztz.hackinglabmobile.fragment.ShareFragment;
import com.oztz.hackinglabmobile.fragment.SpeakerFragment;
import com.oztz.hackinglabmobile.fragment.TeamsFragment;
import com.oztz.hackinglabmobile.fragment.VotingFragment;
import com.oztz.hackinglabmobile.helper.App;

import java.util.Calendar;

public class MainActivity extends ActionBarActivity implements
        NavigationDrawerFragment.NavigationDrawerCallbacks{

    private NavigationDrawerFragment mNavigationDrawerFragment;
    private int currentFragmentPosition = 0;
    private long[] tripleTap;

    private CharSequence mTitle;
    int[] titleArray = {
            R.string.navigationTitle_whatsUp,
            R.string.navigationItem_news,
            R.string.navigationItem_share,
            R.string.navigationTitle_conference,
            R.string.navigationItem_conference,
            R.string.navigationItem_agenda,
            R.string.navigationItem_speaker,
            R.string.navigationTitle_challenge,
            R.string.navigationItem_voting,
            R.string.navigationItem_scoring,
            R.string.navigationItem_challenges,
            R.string.navigationItem_teams
    };

    @Override
    protected void onCreate(Bundle savedInstanceState) {
        App.loadVariables();
        tripleTap = new long[3];
        super.onCreate(savedInstanceState);
        setContentView(R.layout.activity_main);
        mNavigationDrawerFragment = (NavigationDrawerFragment) getSupportFragmentManager()
                .findFragmentById(R.id.navigation_drawer);
        mTitle = getTitle();

        mNavigationDrawerFragment.setUp(R.id.navigation_drawer,
                (DrawerLayout) findViewById(R.id.drawer_layout));
        getFragmentFromIntent();
    }

    @Override
    public void onNavigationDrawerItemSelected(int position) {
        Log.d("DEBUG", "onNavigationDrawerItemSelected("+ String.valueOf(position) +")");
        loadFragment(position);
    }

    public void loadFragment(int position){
        Log.d("DEBUG", "loadFragment(" + String.valueOf(position) + ")");
        Fragment currentFragment;
        switch(position){
            case 0:
                currentFragment = MainFragment.newInstance(position+1);
                break;
            case 1:
                currentFragment = MainFragment.newInstance(position+1);
                break;
            case 2:
                currentFragment = ShareFragment.newInstance(position+1);
                break;
            case 4:
                currentFragment = ConferenceFragment.newInstance(position + 1);
                break;
            case 5:
                currentFragment = AgendaTabHolderFragment.newInstance(position + 1);
                break;
            case 6:
                currentFragment = SpeakerFragment.newInstance(position + 1);
                break;
            case 8:
                currentFragment = VotingFragment.newInstance(position + 1);
                break;
            case 9:
                currentFragment = ScoringFragment.newInstance(position + 1);
                break;
            case 10:
                currentFragment = ChallengesFragment.newInstance(position + 1);
                break;
            case 11:
                currentFragment = TeamsFragment.newInstance(position + 1);
                break;
            default:
                currentFragment = new MainFragment();
        }
        FragmentManager fragmentManager = getSupportFragmentManager();
            fragmentManager
                    .beginTransaction()
                    .replace(R.id.container,
                            currentFragment)
                    .commit();

        currentFragmentPosition = position;

        Log.d("DEBUG", "BackstackCount: "  + fragmentManager.getBackStackEntryCount());
    }

    private void getFragmentFromIntent(){
        Intent intent = getIntent();
        int index = intent.getIntExtra("fragmentIndex", 0);
        loadFragment(index);
    }

    public void onSectionAttached(int number) {
        Log.d("DEBUG", "onSectionAttached("+ String.valueOf(number) +")");
        mTitle = getResources().getString(titleArray[number-1]);
    }

    public void restoreActionBar() {
        ActionBar actionBar = getSupportActionBar();
        actionBar.setNavigationMode(ActionBar.NAVIGATION_MODE_STANDARD);
        actionBar.setDisplayShowTitleEnabled(true);
        actionBar.setTitle(mTitle);
    }

    @Override
    public boolean onCreateOptionsMenu(Menu menu) {
        Log.d("DEBUG", "onCreateOptionsMenu()");
        if (!mNavigationDrawerFragment.isDrawerOpen()) {
            // Only show items in the action bar relevant to this screen
            // if the drawer is not showing. Otherwise, let the drawer
            // decide what to show in the action bar.
            getMenuInflater().inflate(R.menu.main, menu);
            restoreActionBar();
            return true;
        }
        return super.onCreateOptionsMenu(menu);
    }

    @Override
    public void onBackPressed(){
        if(currentFragmentPosition > 1){
            loadFragment(0);
        } else {
            super.onBackPressed();
        }
    }

    @Override
    public boolean onOptionsItemSelected(MenuItem item){
        if(item.getItemId() == R.id.enable_qrCode){
            int minIndex = 0;
            for(int i=1; i<tripleTap.length; i++){
                if(tripleTap[i] < tripleTap[minIndex]){
                    minIndex = i;
                }
            }
            tripleTap[minIndex] = Calendar.getInstance().getTimeInMillis();
            //Java Modulo can produce negative numbers
            int index = (((minIndex - 2) % tripleTap.length) + tripleTap.length) % tripleTap.length;
            long diff = tripleTap[minIndex] - tripleTap[index];
            if(diff < 600){
                startScan();
            }
        }

        return super.onOptionsItemSelected(item);
    }

    private void startScan(){
        IntentIntegrator integrator = new IntentIntegrator(this);
        integrator.initiateScan();
    }

    @Override
    public void onSaveInstanceState(Bundle savedInstanceState) {
        super.onSaveInstanceState(savedInstanceState);
        savedInstanceState.putInt("fragmentPosition", currentFragmentPosition);
    }

    @Override
    public void onRestoreInstanceState(Bundle savedInstanceState) {
        super.onRestoreInstanceState(savedInstanceState);
        currentFragmentPosition = savedInstanceState.getInt("fragmentPosition");
        loadFragment(currentFragmentPosition);
    }

    @Override
    protected void onActivityResult(int requestCode, int resultCode, Intent data) {
        super.onActivityResult(requestCode, resultCode, data);
        IntentResult scanResult = IntentIntegrator.parseActivityResult(requestCode, resultCode, data);
        if (scanResult != null && scanResult.getContents() != null) {
            String payload = scanResult.getContents().trim();
            if(payload.contains("jury") && payload.contains(String.valueOf(App.eventId))){
                Toast.makeText(getApplicationContext(),
                        String.format(getResources().getString(R.string.qr_successful_notification), "jury"),
                        Toast.LENGTH_SHORT).show();
            } else if(payload.contains("author") && payload.contains(String.valueOf(App.eventId))){
                Toast.makeText(getApplicationContext(),
                        String.format(getResources().getString(R.string.qr_successful_notification), "author"),
                        Toast.LENGTH_SHORT).show();
            }
            new DbOperator(getApplicationContext()).addQrCode(payload);
        }
    }
}
