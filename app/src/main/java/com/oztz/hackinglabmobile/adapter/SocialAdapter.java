package com.oztz.hackinglabmobile.adapter;

import android.content.Context;
import android.content.Intent;
import android.net.Uri;
import android.util.Log;
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
import com.oztz.hackinglabmobile.businessclasses.Social;
import com.oztz.hackinglabmobile.helper.AuthImageDownloader;

/**
 * Created by Tobi on 25.03.2015.
 */
public class SocialAdapter extends ArrayAdapter {

    ImageLoader imageLoader;
    private final Context context;

    public SocialAdapter(Context context, int resource, Social[] socials) {
        super(context, resource, socials);
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
        TextView name;
        TextView shareText;
        ImageView shareImage;
        int id;
    }

    @Override
    public View getView(int position, View convertView, ViewGroup parent) {
        ViewHolder holder = new ViewHolder();
        View v;
        final Social item = (Social)getItem(super.getCount() - position - 1);

        LayoutInflater inflater = LayoutInflater.from(getContext());
        v = inflater.inflate(R.layout.item_article_textonly, null);
        if (item != null) {
            holder.id = item.socialID;
            if(item.media!= null){
                v = inflater.inflate(R.layout.item_social_with_media, null);
                holder.shareImage = (ImageView) v.findViewById(R.id.social_thumbnail);
                holder.name = (TextView) v.findViewById(R.id.social_title);
                holder.shareText = (TextView) v.findViewById(R.id.social_text);
                imageLoader.displayImage(item.media, holder.shareImage);
                Log.d("DEBUG", "(1)"+ imageLoader.getLoadingUriForView(holder.shareImage));
                holder.shareImage.setOnClickListener(new View.OnClickListener() {
                    @Override
                    public void onClick(View v) {
                        Intent intent = new Intent();
                        intent.setAction(Intent.ACTION_VIEW);
                        intent.setDataAndType(Uri.parse(item.media), "image/*");
                        context.startActivity(intent);
                    }
                });
            }
            else {
                holder.name = (TextView) v.findViewById(R.id.article_textonly_title);
                holder.shareText = (TextView) v.findViewById(R.id.article_textonly_text);
            }
            if (holder.name != null) {
                holder.name.setText(String.valueOf(item.authorName));
            }
            if (holder.shareText != null) {
                holder.shareText.setText(item.text);
            }
        }
        v.setTag(holder);
        return v;
    }
}
