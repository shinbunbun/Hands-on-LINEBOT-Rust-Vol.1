use std::env;

use line_bot_sdk::{
    models::{
        action::{
            CameraAction, CameraRollAction, DatetimePickerAction, LocationAction, MessageAction,
            PostbackAction, URIAction,
        },
        message::{
            audio::AudioMessage, image::ImageMessage, imagemap::ImagemapMessage,
            location::LocationMessage, stamp::StampMessage, text::TextMessage, video::VideoMessage,
        },
        message::{
            flex::{
                FlexBlockStyle, FlexBox, FlexBoxComponent, FlexBubble, FlexBubbleStyles,
                FlexButton, FlexCarousel, FlexContainer, FlexHero, FlexImage, FlexMessage,
                FlexSeparator, FlexText,
            },
            imagemap::ImagemapURIAction,
            quick_reply::{QuickReply, QuickReplyItem},
            template::{
                buttons::ButtonsTemplate,
                carousel::{self, CarouselTemplate},
                confirm::ConfirmTemplate,
                image_carousel::{self, ImageCarouselTemplate},
                TemplateMessage,
            },
            MessageObject,
        },
        webhook_event::{Event, Text},
    },
    Client,
};

use crate::{error::AppError, news, weather};

pub async fn text_event(
    client: &Client,
    event: &Event,
    message: &Text,
) -> Result<Option<Vec<MessageObject>>, AppError> {
    let messages: Vec<MessageObject> = match message.text.as_str() {
        "こんにちは" => vec![
            TextMessage::builder().text("こんにちは世界").build().into(),
        ],
        "複数メッセージ" => vec![
            TextMessage::builder().text("Hello, user").build().into(),
            TextMessage::builder().text("May I help you?").build().into(),
        ],
        "クイックリプライ" => vec![
            TextMessage::builder()
            .text("クイックリプライ（以下のアクションはクイックリプライ専用で、他のメッセージタイプでは使用できません）")
            .quick_reply(
                QuickReply::builder()
                .items(
                    vec![
                        QuickReplyItem::builder()
                        .action(CameraAction::builder().label("カメラを開く").build().into())
                        .build(),
                        QuickReplyItem::builder()
                        .action(CameraRollAction::builder().label("カメラロールを開く").build().into())
                        .build(),
                        QuickReplyItem::builder()
                        .action(LocationAction::builder().label("位置情報画面を開く").build().into())
                        .build(),
                    ],
                ).build()
            ).build().into(),

        ],
        "スタンプメッセージ" => vec![
            StampMessage::builder()
            .package_id("446")
            .sticker_id("1988")
            .build()
            .into(),
        ],
        "画像メッセージ" => vec![
            ImageMessage::builder()
            .original_content_url("https://shinbunbun.info/images/photos/7.jpeg")
            .preview_image_url("https://shinbunbun.info/images/photos/7.jpeg")
            .build()
            .into()
        ],
        "音声メッセージ" => vec![
            AudioMessage::builder()
            .original_content_url("https://github.com/shinbunbun/aizuhack-bot/blob/master/media/demo.m4a?raw=true")
            .duration(6000)
            .build()
            .into()
        ],
        "動画メッセージ" => vec![
            VideoMessage::builder()
            .original_content_url("https://github.com/shinbunbun/aizuhack-bot/blob/master/media/demo.mp4?raw=true")
            .preview_image_url("https://raw.githubusercontent.com/shinbunbun/aizuhack-bot/master/media/thumbnail.jpg?raw=true")
            .build()
            .into()
        ],
        "位置情報メッセージ" => vec![
            LocationMessage::builder()
            .title("my location")
            .address("〒160-0004 東京都新宿区四谷一丁目6番1号")
            .latitude(35.687574)
            .longitude(139.72922)
            .build()
            .into()
        ],
        "イメージマップメッセージ" => vec![
            ImagemapMessage::builder()
            .base_url("https://youkan-storage.s3.ap-northeast-1.amazonaws.com/ubic_bunbun")
            .alt_text("This is an imagemap")
            .base_size(1040, 597)
            .actions(vec![
                ImagemapURIAction::builder()
                .link_uri("https://www.u-aizu.ac.jp/intro/faculty/ubic/")
                .area(26, 113, 525, 170)
                .build()
                .into(),
                ImagemapURIAction::builder()
                .link_uri("https://shinbunbun.info/about/")
                .area(33, 331, 780, 177)
                .build()
                .into(),
                ImagemapURIAction::builder()
                .link_uri("https://www.u-aizu.ac.jp/")
                .area(939, 484, 94, 105)
                .build()
                .into(),
            ])
            .build()
            .into(),
        ],
        "ボタンテンプレート" => vec![
            TemplateMessage::builder()
            .alt_text("ボタンテンプレート")
            .template(
                ButtonsTemplate::builder()
                .text("ボタンだお")
                .default_action(
                    URIAction::builder()
                    .uri("https://shinbunbun.info/images/photos/")
                    .build()
                    .into()
                )
                .actions(vec![
                    PostbackAction::builder()
                    .label("ポストバックアクション")
                    .data("button-postback")
                    .build()
                    .into(),
                    MessageAction::builder()
                    .label("メッセージアクション")
                    .text("button-message")
                    .build()
                    .into(),
                    URIAction::builder()
                    .label("URIアクション")
                    .uri("https://shinbunbun.info/")
                    .build()
                    .into(),
                    DatetimePickerAction::builder()
                    .data("button-date")
                    .mode("datetime")
                    .label("日時選択アクション")
                    .initial("2021-06-01t00:00")
                    .max("2022-12-31t23:59")
                    .min("2021-06-01t00:00")
                    .build()
                    .into(),
                ])
                .build()
                .into(),
            )
            .build()
            .into(),
        ],
        "確認テンプレート" => vec![
            TemplateMessage::builder()
            .alt_text("確認テンプレート")
            .template(
                ConfirmTemplate::builder()
                .text("確認テンプレート")
                .actions(vec![
                    MessageAction::builder()
                    .label("はい")
                    .text("yes")
                    .build()
                    .into(),
                    MessageAction::builder()
                    .label("いいえ")
                    .text("no")
                    .build()
                    .into(),
                ])
                .build()
                .into(),
            )
            .build()
            .into(),
        ],
        "カルーセルテンプレート" => vec![
            TemplateMessage::builder()
            .alt_text("カルーセルテンプレート")
            .template(
                CarouselTemplate::builder()
                .columns(vec![
                    carousel::Column::builder()
                    .text("説明1")
                    .actions(vec![
                        PostbackAction::builder()
                        .label("ポストバック")
                        .data("postback-carousel-1")
                        .build()
                        .into(),
                        URIAction::builder()
                        .label("URIアクション")
                        .uri("https://shinbunbun.info/")
                        .build()
                        .into(),
                    ])
                    .default_action(
                        URIAction::builder()
                        .uri("https://shinbunbun.info/images/photos/")
                        .build()
                        .into()
                    )
                    .thumbnail_image_url("https://shinbunbun.info/images/photos/7.jpeg")
                    .image_background_color("#FFFFFF")
                    .title("タイトル1")
                    .build(),
                    carousel::Column::builder()
                    .text("説明2")
                    .actions(vec![
                        PostbackAction::builder()
                        .label("ポストバック")
                        .data("postback-carousel-2")
                        .build()
                        .into(),
                        URIAction::builder()
                        .label("URIアクション")
                        .uri("https://shinbunbun.info/")
                        .build()
                        .into(),
                    ])
                    .default_action(
                        URIAction::builder()
                        .uri("https://shinbunbun.info/images/photos/")
                        .build()
                        .into()
                    )
                    .thumbnail_image_url("https://shinbunbun.info/images/photos/10.jpeg")
                    .image_background_color("#FFFFFF")
                    .title("タイトル2")
                    .build(),
                ])
                .build()
                .into()
            )
            .build()
            .into(),
        ],
        "画像カルーセルテンプレート"=> vec![
            TemplateMessage::builder()
            .alt_text("画像カルーセルテンプレート")
            .template(
                ImageCarouselTemplate::builder()
                .columns(vec![
                    image_carousel::Column::builder()
                    .image_url("https://shinbunbun.info/images/photos/4.jpeg")
                    .action(
                        PostbackAction::builder()
                        .label("ポストバック")
                        .data("image-carousel-1")
                        .build()
                        .into(),
                    )
                    .build(),
                    image_carousel::Column::builder()
                    .image_url("https://shinbunbun.info/images/photos/5.jpeg")
                    .action(
                        MessageAction::builder()
                        .label("メッセージ")
                        .text("text")
                        .build()
                        .into(),
                    )
                    .build(),
                    image_carousel::Column::builder()
                    .image_url("https://shinbunbun.info/images/photos/7.jpeg")
                    .action(
                        URIAction::builder()
                        .label("URIアクション")
                        .uri("https://shinbunbun.info/")
                        .build()
                        .into(),
                    )
                    .build(),
                ])
                .build()
                .into()
            )
            .build()
            .into(),
        ],
        "Flex Message" => {
            vec![
                FlexMessage::builder()
                .alt_text("Flex Message")
                .contents(
                    FlexBubble::builder()
                    .header(
                        FlexBox::builder()
                        .layout("vertical")
                        .contents(vec![
                            FlexText::builder()
                            .text("Flex Message")
                            .color("#FFFFFF")
                            .weight("bold")
                            .build()
                            .into(),
                        ])
                        .build()
                    )
                    .hero(
                        FlexImage::builder()
                        .url("https://pbs.twimg.com/profile_images/1236928986212478976/wDa51i9T_400x400.jpg")
                        .size("xl")
                        .build()
                        .into(),
                    )
                    .body(
                        FlexBox::builder()
                        .layout("vertical")
                        .contents(vec![
                            FlexText::builder()
                            .text("しんぶんぶん")
                            .size("xl")
                            .weight("bold")
                            .align("center")
                            .build()
                            .into(),
                            FlexText::builder()
                            .text("会津大学学部一年")
                            .align("center")
                            .build()
                            .into(),
                            FlexSeparator::builder()
                            .margin("md")
                            .build()
                            .into(),
                            FlexBox::builder()
                            .layout("vertical")
                            .contents(vec![
                                FlexButton::builder()
                                .action(
                                    URIAction::builder()
                                    .label("ホームページ")
                                    .uri("https://shinbunbun.info/")
                                    .build()
                                    .into(),
                                )
                                .style("primary")
                                .offset_bottom("10px")
                                .build()
                                .into(),
                                FlexButton::builder()
                                .action(
                                    URIAction::builder()
                                    .label("Twitter")
                                    .uri("https://twitter.com/shinbunbun_")
                                    .build()
                                    .into(),
                                )
                                .style("primary")
                                .color("#1DA1F2")
                                .build()
                                .into(),
                            ])
                            .padding_top("10px")
                            .build()
                            .into(),
                        ])
                        .build()
                    )
                    .styles(
                        FlexBubbleStyles::builder()
                        .header(
                            FlexBlockStyle::builder()
                            .background_color("#008282")
                            .build()
                        )
                        .build()
                    )
                    .build()
                    .into(),
                )
                .build()
                .into()
            ]
        },
        "プロフィール"=>{
            let profile = client.profile(event.source.user_id.as_ref().ok_or_else(|| AppError::BadRequest("userId not found".to_string()))?).await.map_err(AppError::LineBotSdkError)?;
            vec![
                TextMessage::builder()
                .text(&format!("あなたの名前: {}\nユーザーID: {}\nプロフィール画像のURL: {}\nステータスメッセージ: {}", profile.display_name, profile.user_id, profile.picture_url, profile.status_message.unwrap_or_else(|| "未設定".to_string())))
                .build()
                .into(),
            ]
        },
        "ここはどこ"=>{
            if event.source.type_field=="user"{
                vec![
                    TextMessage::builder()
                    .text("ここは個チャだよ!")
                    .build()
                    .into(),
                ]
            } else if event.source.type_field=="group"{
                vec![
                    TextMessage::builder()
                    .text("ここはグループだよ!")
                    .build()
                    .into(),
                ]
            } else {
                vec![
                    TextMessage::builder()
                    .text("非対応のソースです")
                    .build()
                    .into(),
                ]
            }
        },
        "おはよう" => vec![
            TextMessage::builder().text("Good Morning!").build().into(),
        ],
        "予定" => vec![
            TextMessage::builder()
            .text("予定を知りたい曜日を選んでください")
            .quick_reply(
                QuickReply::builder()
                .items(vec![
                    QuickReplyItem::builder()
                    .action(
                        MessageAction::builder()
                        .label("月曜日")
                        .text("月曜日の予定")
                        .build()
                        .into()
                    )
                    .build(),
                    QuickReplyItem::builder()
                    .action(
                        MessageAction::builder()
                        .label("火曜日")
                        .text("火曜日の予定")
                        .build()
                        .into()
                    )
                    .build(),
                    QuickReplyItem::builder()
                    .action(
                        MessageAction::builder()
                        .label("水曜日")
                        .text("水曜日の予定")
                        .build()
                        .into()
                    )
                    .build(),
                    QuickReplyItem::builder()
                    .action(
                        MessageAction::builder()
                        .label("木曜日")
                        .text("木曜日の予定")
                        .build()
                        .into()
                    )
                    .build(),
                    QuickReplyItem::builder()
                    .action(
                        MessageAction::builder()
                        .label("金曜日")
                        .text("金曜日の予定")
                        .build()
                        .into()
                    )
                    .build(),
                ])
                .build()
            )
            .build()
            .into(),
        ],
        "月曜日の予定" => vec![
            TextMessage::builder()
            .text("1. 力学\n2. 力学\n3. 力学\n4. 力学\n5. 微積分\n6. 微積分")
            .build()
            .into(),
        ],
        "火曜日の予定" => vec![
            TextMessage::builder()
            .text("3. プログ0\n4. プログ0\n5. プログ0\n6. プログ0\n7. Eng2\n8. プログ0\n9. 経済学\n10. 経済学")
            .build()
            .into(),
        ],
        "水曜日の予定" => vec![
            TextMessage::builder()
            .text("3. 微積分\n4. 微積分")
            .build()
            .into(),
        ],
        "木曜日の予定" => vec![
            TextMessage::builder()
            .text("1. 力学\n2. 力学\n3. 力学\n4. 力学\n5. 微積分\n6. 微積分\n7. 体育実技\n8. 体育実技")
            .build()
            .into(),
        ],
        "金曜日の予定" => vec![
            TextMessage::builder()
            .text("3. プログ0\n4. プログ0\n5. プログ0\n6. プログ0\n7. Eng2\n8. プログ0\n9. 経済学\n10. 経済学")
            .build()
            .into(),
        ],
        "天気予報" => {
            let weather_api_res = reqwest::get("https://www.jma.go.jp/bosai/forecast/data/forecast/070000.json").await.map_err(AppError::ReqwestError)?.json::<weather::Root>().await.map_err(AppError::ReqwestError)?;
            let text = format!("【天気予報】\n\n{}: {}\n{}: {}\n{}: {}",
             weather_api_res[0].time_series[0].time_defines[0],
             weather_api_res[0].time_series[0].areas[2].weathers.as_ref().unwrap_or(&vec!["".to_string()])[0],
             weather_api_res[0].time_series[0].time_defines[1],
             weather_api_res[0].time_series[0].areas[2].weathers.as_ref().unwrap_or(&vec!["".to_string()])[1],
             weather_api_res[0].time_series[0].time_defines[2],
             weather_api_res[0].time_series[0].areas[2].weathers.as_ref().unwrap_or(&vec!["".to_string()])[2],
            );
            vec![TextMessage::builder().text(&text).build().into()]
        },
        "ニュース1" => {
            let client = reqwest::Client::builder()
            .user_agent(concat!(
                env!("CARGO_PKG_NAME"),
                "/",
                env!("CARGO_PKG_VERSION"),
            ))
            .build()
            .map_err(AppError::ReqwestError)?;

            let news_api_res = client.get(&format!("https://newsapi.org/v2/top-headlines?country=jp&apiKey={}&pageSize=5", env::var("NEWS_API_KEY").map_err(AppError::EnvError)?))
            .send()
            .await
            .map_err(AppError::ReqwestError)?
            .json::<news::Root>()
            .await
            .map_err(AppError::ReqwestError)?;

            let mut message: Vec<MessageObject> = Vec::new();

            let articles = news_api_res.articles;
            for article in articles {
                message.push(
                    TextMessage::builder()
                    .text(&format!(
                        "【画像URL】: {}\n【タイトル】: {}\n【公開日】: {}\n【概要】: {}\n【記事のURL】: {}\n【掲載元】: {}",
                        article.url_to_image.unwrap_or_else(||"null".to_string()),
                        article.title.unwrap_or_else(|| "null".to_string()),
                        article.published_at.unwrap_or_else(|| "null".to_string()),
                        article.description.unwrap_or_else(||"null".to_string()),
                        article.url.unwrap_or_else(||"null".to_string()),
                        article.source.name.unwrap_or_else(||"null".to_string())))
                    .build().into()
                );
            }
            message
        },
        "ニュース2" => {
            let client = reqwest::Client::builder()
            .user_agent(concat!(
                env!("CARGO_PKG_NAME"),
                "/",
                env!("CARGO_PKG_VERSION"),
            ))
            .build()
            .map_err(AppError::ReqwestError)?;

            let news_api_res = client.get(&format!("https://newsapi.org/v2/top-headlines?country=jp&apiKey={}&pageSize=5", env::var("NEWS_API_KEY").map_err(AppError::EnvError)?))
            .send()
            .await
            .map_err(AppError::ReqwestError)?
            .json::<news::Root>()
            .await
            .map_err(AppError::ReqwestError)?;

            let mut message = FlexMessage::builder()
            .alt_text("ニュース一覧")
            .contents(
                FlexContainer::Carousel(
                    FlexCarousel::builder()
                    .contents(
                        vec![]
                    ).build()
                )
            ).build();

            let articles = news_api_res.articles;
            for article in articles {
                if let FlexContainer::Carousel(ref mut carousel) = message.contents {
                    carousel.contents.push(FlexBubble::builder()
                    .size("kilo")
                    .hero(
                        FlexHero::Image(
                            FlexImage::builder()
                            .url(&article.url_to_image.unwrap_or_else(||"https://raw.githubusercontent.com/shinbunbun/aizuhack-bot/master/media/imagemap.png".to_owned()))
                            .size("full")
                            .aspect_mode("cover")
                            .build()
                            .into()
                        )

                    )
                    .body(
                        FlexBox::builder()
                        .layout("vertical")
                        .contents(
                            vec![
                                FlexBoxComponent::Text(
                                    Box::new(FlexText::builder()
                                    .weight("bold")
                                    .size("sm")
                                    .wrap(true)
                                    .text(&article.title.unwrap_or_else(|| "No title".to_string()))
                                    .build())
                                ),
                                FlexBoxComponent::Text(
                                    Box::new(FlexText::builder()
                                    .size("xs")
                                    .wrap(true)
                                    .text(&article.published_at.unwrap_or_else(|| "No published_at".to_string()))
                                    .build())
                                ),
                                FlexBoxComponent::Text(
                                    Box::new(FlexText::builder()
                                    .size("sm")
                                    .wrap(true)
                                    .text(&article.description.unwrap_or_else(|| "No description".to_string()))
                                    .build())
                                ),
                            ]
                        )
                        .spacing("md")
                        .build()
                    )
                    .footer(
                        FlexBox::builder()
                        .layout("vertical")
                        .contents(
                            vec![
                                FlexBoxComponent::Button(
                                    FlexButton::builder()
                                    .action(
                                        URIAction::builder()
                                        .uri(&article.url.unwrap_or_else(|| "https://example.com".to_string()))
                                        .label(&article.source.name.unwrap_or_else(|| "No source name".to_string()))
                                        .build()
                                        .into()
                                    )
                                    .style("primary")
                                    .build()
                                    .into()
                                )
                            ]
                        )
                        .build()
                    ).build())
                }
            }
            vec![message.into()]
        },
        _ => vec![
                TextMessage::builder()
                .text(&format!("受け取ったメッセージ: {}\nそのメッセージの返信には対応してません...", message.text))
                .build()
                .into(),
        ],
    };
    Ok(Some(messages))
}
