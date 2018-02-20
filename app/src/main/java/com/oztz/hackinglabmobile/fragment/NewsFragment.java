package com.oztz.hackinglabmobile.fragment;

import android.os.Bundle;
import android.support.v4.app.Fragment;
import android.util.Log;
import android.view.LayoutInflater;
import android.view.View;
import android.view.ViewGroup;
import android.widget.ListView;
import android.widget.Toast;

import com.google.gson.Gson;
import com.oztz.hackinglabmobile.R;
import com.oztz.hackinglabmobile.adapter.NewsAdapter;
import com.oztz.hackinglabmobile.businessclasses.News;
import com.oztz.hackinglabmobile.helper.App;
import com.oztz.hackinglabmobile.helper.HttpResult;
import com.oztz.hackinglabmobile.helper.RequestTask;

import java.util.Arrays;
import java.util.Comparator;

/**
 * Created by Tobi on 20.03.2015.
 */
public class NewsFragment extends Fragment implements HttpResult {

    private ListView newsListView;
    private RequestTask requestTask;
    View footer;

    @Override
    public void onCreate(Bundle savedInstanceState) {
        Log.d("DEBUG", "NewsFragment.onCreate()");
        super.onCreate(savedInstanceState);
    }

    @Override
    public View onCreateView(LayoutInflater inflater, ViewGroup container,
                             Bundle savedInstanceState)
    {
        Log.d("DEBUG", "NewsFragment.onCreateView()");
        View view = inflater.inflate(R.layout.fragment_news, container, false);
        newsListView = (ListView) view.findViewById(R.id.news_listview);

        String url = getResources().getString(R.string.rootURL) + "event/" +
                String.valueOf(App.eventId) + "/news/newest";
        updateView(App.db.getContentFromDataBase(url), "db");
        requestTask = new RequestTask(this);
        requestTask.execute(url, "newest");

        return view;
    }

    @Override
    public void onTaskCompleted(String result, String requestCode) {
        if(result != null) {
            updateView(result, requestCode);
        } else {
            Toast.makeText(getActivity().getApplicationContext(), "Error Getting Data",Toast.LENGTH_SHORT);
        }
    }

    private void loadMoreData(){
        new RequestTask(this).execute(getResources().getString(R.string.rootURL) + "event/" +
                String.valueOf(App.eventId) + "/news", "all");
    }

    @Override
    public void onDestroyView(){
        super.onDestroyView();
        Log.d("DEBUG", "NewsFragment.onDestroyView()");
        requestTask.cancel(true);
    }

    private void updateView(String json, String requestCode){
        if(json != null){
            try {
                News[] news = new Gson().fromJson(json, News[].class);
                Arrays.sort(news, new Comparator<News>() {
                    @Override
                    public int compare(News lhs, News rhs) {
                        return lhs.newsID - rhs.newsID;
                    }
                });
                if(requestCode.equals("newest")) {
                    if (newsListView.getFooterViewsCount() == 0 && news.length >= App.newestSelectLimit) {
                        LayoutInflater inflater = LayoutInflater.from(getActivity().getApplicationContext());
                        footer = inflater.inflate(R.layout.item_load_more_data, null);
                        footer.setOnClickListener(new View.OnClickListener() {
                            @Override
                            public void onClick(View v) {
                                loadMoreData();
                            }
                        });
                        newsListView.addFooterView(footer);
                    }
                    newsListView.setAdapter(new NewsAdapter(getActivity(), R.layout.item_article_textonly, news));
                } else if(requestCode.equals("all")){
                    int scrollPosition = newsListView.getFirstVisiblePosition();
                    newsListView.removeFooterView(footer);
                    newsListView.setAdapter(new NewsAdapter(getActivity(), R.layout.item_article_textonly, news));
                    newsListView.setSelectionFromTop(scrollPosition+1, 0);
                } else if(requestCode.equals("db")){
                    newsListView.setAdapter(new NewsAdapter(getActivity(), R.layout.item_article_textonly, news));
                }
            } catch(Exception e){
                Toast.makeText(getActivity().getApplicationContext(), "Error loading Data",Toast.LENGTH_SHORT);
            }
        }
    }
}
