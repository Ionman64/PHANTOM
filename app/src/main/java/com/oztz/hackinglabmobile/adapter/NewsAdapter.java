package com.oztz.hackinglabmobile.adapter;

import android.content.Context;
import android.view.LayoutInflater;
import android.view.View;
import android.view.ViewGroup;
import android.widget.ArrayAdapter;
import android.widget.ImageView;
import android.widget.TextView;

import com.nostra13.universalimageloader.core.DisplayImageOptions;
import com.nostra13.universalimageloader.core.ImageLoader;
import com.nostra13.universalimageloader.core.ImageLoaderConfiguration;
import com.oztz.hackinglabmobile.R;
import com.oztz.hackinglabmobile.businessclasses.News;
import com.oztz.hackinglabmobile.helper.AuthImageDownloader;

/**
 * Created by Tobi on 25.03.2015.
 */
public class NewsAdapter extends ArrayAdapter {

    ImageLoader imageLoader;
    private final Context context;

    public NewsAdapter(Context context, int resource, News[] news) {
        super(context, resource, news);
        this.context = context;
        imageLoader = ImageLoader.getInstance();
        ImageLoaderConfiguration config = new ImageLoaderConfiguration.Builder(context)
                .imageDownloader(new AuthImageDownloader(context, 5000, 20000))
                .diskCacheFileCount(50)
                .defaultDisplayImageOptions(new DisplayImageOptions.Builder()
                        .cacheInMemory(true)
                        .cacheOnDisk(true).build())
                .build();
        imageLoader.init(config);
    }

    private static class ViewHolder {
        TextView title;
        TextView text;
        ImageView newsImage;
        int id;
    }

    @Override
    public View getView(int position, View convertView, ViewGroup parent) {

        ViewHolder holder = new ViewHolder();
        View v;
        News item = (News)getItem(super.getCount() - position - 1);

        LayoutInflater inflater = LayoutInflater.from(getContext());
        v = inflater.inflate(R.layout.item_article_textonly, null);

        if (item != null) {
            holder.id = item.newsID;
            if(item.media != null){
                v = inflater.inflate(R.layout.item_social_with_media, null);
                holder.newsImage = (ImageView) v.findViewById(R.id.social_thumbnail);
                holder.title = (TextView) v.findViewById(R.id.social_title);
                holder.text = (TextView) v.findViewById(R.id.social_text);
                imageLoader.displayImage(item.media, holder.newsImage);
            }
            else {
                holder.title = (TextView) v.findViewById(R.id.article_textonly_title);
                holder.text = (TextView) v.findViewById(R.id.article_textonly_text);
            }
            if (holder.title != null) {
                holder.title.setText(String.valueOf(item.title));
            }
            if (holder.text != null) {
                holder.text.setText(item.text);
            }
        }
        v.setTag(holder);
        return v;
    }
}
