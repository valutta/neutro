macro_rules! мяу_предмет {
    ($item:item) => { $item };
}

мяу_предмет! { use crate::{Context, Error, state::{МяуДанные, МяуЧерновикФлага}}; }
use poise::serenity_prelude as serenity;
use serde_json::json;

macro_rules! мяу {
    ($lang:expr, $key:expr) => {
        мяяяяяу_20__($lang, $key)
    };
}

macro_rules! меееов {
    ($who:ident <- $what:expr) => {
        let $who = $what;
    };
    (mut $who:ident <- $what:expr) => {
        let mut $who = $what;
    };
    ($who:ident : $kind:ty = $what:expr) => {
        let $who: $kind = $what;
    };
    (mut $who:ident : $kind:ty = $what:expr) => {
        let mut $who: $kind = $what;
    };
}

macro_rules! мяу_если {
    ($cond:expr => $then:block) => {
        if $cond $then
    };
    ($cond:expr => $then:block иначе $else:block) => {
        if $cond $then else $else
    };
}

macro_rules! мяу_если_пусть {
    (Some(mut $who:ident) = $what:expr => $then:block иначе $else:block) => {
        if let Some(mut $who) = $what $then else $else
    };
    (Some($who:ident) = $what:expr => $then:block иначе $else:block) => {
        if let Some($who) = $what $then else $else
    };
    (Ok($who:ident) = $what:expr => $then:block иначе $else:block) => {
        if let Ok($who) = $what $then else $else
    };
    (Err($who:ident) = $what:expr => $then:block иначе $else:block) => {
        if let Err($who) = $what $then else $else
    };
}

#[poise::command(slash_command, prefix_command, rename = "terminal", subcommands("мяяяяяу_02__", "мяяяяяу_03__", "мяяяяяу_04__", "мяяяяяу_05__", "мяяяяяу_06__", "мяяяяяу_07__", "мяяяяяу_08__", "мяяяяяу_09__", "мяяяяяу_10__", "мяяяяяу_11__", "мяяяяяу_12__", "мяяяяяу_13__", "мяяяяяу_14__", "мяяяяяу_15__", "мяяяяяу_16__"))]
pub async fn мяяяяяу_00__(
    ctx: Context<'_>,
) -> Result<(), Error> {
    меееов!(мяу_язычок <- мяяяяяу_18__(&ctx).await);
    crate::v2_components::мяу_v2_посылка_90__(ctx, мяяяяяу_23__(&мяу_язычок, "help")).await?;
    Ok(())
}

async fn мяяяяяу_01__(ctx: &Context<'_>) -> bool {
    let Some(guild_id) = ctx.guild_id() else {
        return false;
    };
    if let Some(guild) = ctx.cache().guild(guild_id) {
        if guild.owner_id == ctx.author().id {
            return true;
        }
    }
    if let Ok(member) = guild_id.member(ctx.http(), ctx.author().id).await {
        if let Ok(perms) = member.permissions(ctx.cache()) {
            if perms.administrator() {
                return true;
            }
        }
        if let Some(role_id) = ctx.data().мяу_роль_стаффа_39__(Some(guild_id)).await {
            if member.roles.contains(&role_id) {
                return true;
            }
        }
    }
    меееов!(мяу_автор <- ctx.author().id.to_string());
    меееов!(мяу_список <- ctx.data().terminal_whitelist.read().await);
    мяу_список.contains(&мяу_автор)
}

#[poise::command(slash_command, prefix_command, rename = "wl")]
pub async fn мяяяяяу_02__(
    ctx: Context<'_>,
    #[description = "Action: add, rm, list"] action: String,
    #[description = "Target User ID"] target: Option<String>,
) -> Result<(), Error> {
    меееов!(мяу_язычок <- мяяяяяу_18__(&ctx).await);
    мяу_если!(!мяяяяяу_01__(&ctx).await => {
        crate::v2_components::мяу_v2_посылка_90__(ctx, serde_json::json!([{ "type": 17, "components": [{ "type": 10, "content": мяу!(&мяу_язычок, "unauthorized") }] }])).await?;
        return Ok(());
    });

    match action.as_str() {
        "list" => {
            меееов!(мяу_белый_список <- ctx.data().terminal_whitelist.read().await);
            меееов!(мяу_простыня <- мяу_если!(мяу_белый_список.is_empty() => {
                мяу!(&мяу_язычок, "empty").to_string()
            } иначе {
                мяу_белый_список.iter().map(|id| format!("<@{}>", id)).collect::<Vec<_>>().join(", ")
            }));
            crate::v2_components::мяу_v2_посылка_90__(ctx, serde_json::json!([{ "type": 17, "components": [{ "type": 10, "content": мяу!(&мяу_язычок, "wl_list").replace("{list}", &мяу_простыня) }] }])).await?;
        }
        "add" => {
            мяу_если_пусть!(Some(mut мяу_id) = target => {
                мяу_id = мяу_id.chars().filter(|c| c.is_digit(10)).collect();
                меееов!(мяу_добавилось <- {
                    меееов!(mut мяу_белый_список <- ctx.data().terminal_whitelist.write().await);
                    мяу_если!(мяу_белый_список.contains(&мяу_id) => {
                        false
                    } иначе {
                        мяу_белый_список.push(мяу_id);
                        true
                    })
                });
                мяу_если!(!мяу_добавилось => {
                    crate::v2_components::мяу_v2_посылка_90__(ctx, serde_json::json!([{ "type": 17, "components": [{ "type": 10, "content": мяу!(&мяу_язычок, "wl_exists") }] }])).await?;
                } иначе {
                    ctx.data().мяу_сохрани_whitelist_27__().await;
                    crate::v2_components::мяу_v2_посылка_90__(ctx, serde_json::json!([{ "type": 17, "components": [{ "type": 10, "content": мяу!(&мяу_язычок, "wl_added") }] }])).await?;
                });
            } иначе {
                crate::v2_components::мяу_v2_посылка_90__(ctx, serde_json::json!([{ "type": 17, "components": [{ "type": 10, "content": мяу!(&мяу_язычок, "wl_usage_add") }] }])).await?;
            });
        }
        "rm" => {
            мяу_если_пусть!(Some(mut мяу_id) = target => {
                мяу_id = мяу_id.chars().filter(|c| c.is_digit(10)).collect();
                меееов!(мяу_снеслось <- {
                    меееов!(mut мяу_белый_список <- ctx.data().terminal_whitelist.write().await);
                    мяу_если!(мяу_белый_список.contains(&мяу_id) => {
                        мяу_белый_список.retain(|x| x != &мяу_id);
                        true
                    } иначе {
                        false
                    })
                });
                мяу_если!(мяу_снеслось => {
                    ctx.data().мяу_сохрани_whitelist_27__().await;
                    crate::v2_components::мяу_v2_посылка_90__(ctx, serde_json::json!([{ "type": 17, "components": [{ "type": 10, "content": мяу!(&мяу_язычок, "wl_removed") }] }])).await?;
                } иначе {
                    crate::v2_components::мяу_v2_посылка_90__(ctx, serde_json::json!([{ "type": 17, "components": [{ "type": 10, "content": мяу!(&мяу_язычок, "wl_missing") }] }])).await?;
                });
            } иначе {
                crate::v2_components::мяу_v2_посылка_90__(ctx, serde_json::json!([{ "type": 17, "components": [{ "type": 10, "content": мяу!(&мяу_язычок, "wl_usage_rm") }] }])).await?;
            });
        }
        _ => {
            crate::v2_components::мяу_v2_посылка_90__(ctx, serde_json::json!([{ "type": 17, "components": [{ "type": 10, "content": мяу!(&мяу_язычок, "wl_usage") }] }])).await?;
        }
    }

    Ok(())
}


#[poise::command(slash_command, prefix_command, rename = "ap")]
pub async fn мяяяяяу_03__(
    ctx: Context<'_>,
    #[description = "Request Channel ID"] request_id: String,
    #[description = "Approval Channel ID"] approval_id: String,
) -> Result<(), Error> {
    меееов!(мяу_язычок <- мяяяяяу_18__(&ctx).await);
    мяу_если!(!мяяяяяу_01__(&ctx).await => {
        crate::v2_components::мяу_v2_посылка_90__(ctx, serde_json::json!([{ "type": 17, "components": [{ "type": 10, "content": мяу!(&мяу_язычок, "unauthorized") }] }])).await?;
        return Ok(());
    });

    меееов!(мяу_req: String = request_id.chars().filter(|c| c.is_digit(10)).collect());
    меееов!(мяу_ap: String = approval_id.chars().filter(|c| c.is_digit(10)).collect());

    мяу_если!(мяу_req.is_empty() || мяу_ap.is_empty() => {
        crate::v2_components::мяу_v2_посылка_90__(ctx, serde_json::json!([{ "type": 17, "components": [{ "type": 10, "content": мяу!(&мяу_язычок, "invalid_channel_ids") }] }])).await?;
        return Ok(());
    });

    {
        меееов!(mut мяу_каналы <- ctx.data().approval_channels.write().await);
        мяу_каналы.insert(мяу_req, мяу_ap);
    }
    ctx.data().мяу_сохрани_approval_28__().await;

    crate::v2_components::мяу_v2_посылка_90__(ctx, serde_json::json!([{ "type": 17, "components": [{ "type": 10, "content": мяу!(&мяу_язычок, "approval_mapped") }] }])).await?;
    Ok(())
}

#[poise::command(slash_command, prefix_command, rename = "fetch")]
pub async fn мяяяяяу_04__(ctx: Context<'_>) -> Result<(), Error> {
    меееов!(мяу_язычок <- мяяяяяу_18__(&ctx).await);
    мяу_если!(!мяяяяяу_01__(&ctx).await => { return Ok(()); });
    меееов!(мяу_путь <- std::path::PathBuf::from("../assets/noorfetch.txt"));
    мяу_если_пусть!(Ok(мяу_текст) = std::fs::read_to_string(мяу_путь) => {
        crate::v2_components::мяу_v2_посылка_90__(ctx, serde_json::json!([{ "type": 17, "components": [{ "type": 10, "content": format!("```\n{}\n```", мяу_текст) }] }])).await?;
    } иначе {
        crate::v2_components::мяу_v2_посылка_90__(ctx, serde_json::json!([{ "type": 17, "components": [{ "type": 10, "content": мяу!(&мяу_язычок, "fetch_missing") }] }])).await?;
    });
    Ok(())
}

#[poise::command(slash_command, prefix_command, rename = "echo")]
pub async fn мяяяяяу_05__(
    ctx: Context<'_>,
    #[description = "Channel"] mut channel: poise::serenity_prelude::GuildChannel,
    #[description = "Description"] description: String,
) -> Result<(), Error> {
    let lang = мяяяяяу_18__(&ctx).await;
    if !мяяяяяу_01__(&ctx).await { return Ok(()); }
    if let Err(e) = channel.edit(ctx.http(), poise::serenity_prelude::EditChannel::new().topic(description)).await {
        crate::v2_components::мяу_v2_посылка_90__(ctx, serde_json::json!([{ "type": 17, "components": [{ "type": 10, "content": мяу!(&lang, "echo_error").replace("{err}", &e.to_string()) }] }])).await?;
    } else {
        crate::v2_components::мяу_v2_посылка_90__(ctx, serde_json::json!([{ "type": 17, "components": [{ "type": 10, "content": мяу!(&lang, "echo_ok") }] }])).await?;
    }
    Ok(())
}

#[poise::command(slash_command, prefix_command, rename = "touch")]
pub async fn мяяяяяу_06__(
    ctx: Context<'_>,
    #[description = "Channel name"] name: String,
    #[description = "Category ID"] category: Option<poise::serenity_prelude::ChannelId>,
) -> Result<(), Error> {
    let lang = мяяяяяу_18__(&ctx).await;
    if !мяяяяяу_01__(&ctx).await { return Ok(()); }
    let guild = ctx.guild_id().ok_or("No guild")?;
    let mut builder = poise::serenity_prelude::CreateChannel::new(name).kind(poise::serenity_prelude::ChannelType::Text);
    if let Some(cat) = category {
        builder = builder.category(cat);
    }
    guild.create_channel(ctx.http(), builder).await?;
    crate::v2_components::мяу_v2_посылка_90__(ctx, serde_json::json!([{ "type": 17, "components": [{ "type": 10, "content": мяу!(&lang, "channel_created") }] }])).await?;
    Ok(())
}

#[poise::command(slash_command, prefix_command, rename = "mkdir")]
pub async fn мяяяяяу_07__(
    ctx: Context<'_>,
    #[description = "Category name"] name: String,
) -> Result<(), Error> {
    let lang = мяяяяяу_18__(&ctx).await;
    if !мяяяяяу_01__(&ctx).await { return Ok(()); }
    let guild = ctx.guild_id().ok_or("No guild")?;
    let builder = poise::serenity_prelude::CreateChannel::new(name).kind(poise::serenity_prelude::ChannelType::Category);
    guild.create_channel(ctx.http(), builder).await?;
    crate::v2_components::мяу_v2_посылка_90__(ctx, serde_json::json!([{ "type": 17, "components": [{ "type": 10, "content": мяу!(&lang, "category_created") }] }])).await?;
    Ok(())
}

#[poise::command(slash_command, prefix_command, rename = "rm")]
pub async fn мяяяяяу_08__(
    ctx: Context<'_>,
    #[description = "Flag"] flag: String,
    #[description = "ID"] id: String,
) -> Result<(), Error> {
    let lang = мяяяяяу_18__(&ctx).await;
    if !мяяяяяу_01__(&ctx).await { return Ok(()); }
    let id_num: u64 = id.chars().filter(|c| c.is_ascii_digit()).collect::<String>().parse().unwrap_or(0);
    if id_num == 0 {
        crate::v2_components::мяу_v2_посылка_90__(ctx, serde_json::json!([{ "type": 17, "components": [{ "type": 10, "content": мяу!(&lang, "invalid_id") }] }])).await?;
        return Ok(());
    }

    if flag == "-m" {
        let msg_id = poise::serenity_prelude::MessageId::new(id_num);
        if let Err(_) = ctx.channel_id().delete_message(ctx.http(), msg_id).await {
            crate::v2_components::мяу_v2_посылка_90__(ctx, serde_json::json!([{ "type": 17, "components": [{ "type": 10, "content": мяу!(&lang, "delete_message_error") }] }])).await?;
        } else {
            crate::v2_components::мяу_v2_посылка_90__(ctx, serde_json::json!([{ "type": 17, "components": [{ "type": 10, "content": мяу!(&lang, "deleted") }] }])).await?;
        }
    } else if flag == "-c" || flag == "-ch" {
        let channel_id = poise::serenity_prelude::ChannelId::new(id_num);
        if let Err(_) = channel_id.delete(ctx.http()).await {
            crate::v2_components::мяу_v2_посылка_90__(ctx, serde_json::json!([{ "type": 17, "components": [{ "type": 10, "content": мяу!(&lang, "delete_channel_error") }] }])).await?;
        } else {
            crate::v2_components::мяу_v2_посылка_90__(ctx, serde_json::json!([{ "type": 17, "components": [{ "type": 10, "content": мяу!(&lang, "deleted") }] }])).await?;
        }
    } else {
        crate::v2_components::мяу_v2_посылка_90__(ctx, serde_json::json!([{ "type": 17, "components": [{ "type": 10, "content": мяу!(&lang, "rm_usage") }] }])).await?;
    }
    Ok(())
}

#[poise::command(slash_command, prefix_command, rename = "mv")]
pub async fn мяяяяяу_09__(
    ctx: Context<'_>,
    #[description = "Channel"] mut channel: poise::serenity_prelude::GuildChannel,
    #[description = "Category"] category: poise::serenity_prelude::ChannelId,
) -> Result<(), Error> {
    let lang = мяяяяяу_18__(&ctx).await;
    if !мяяяяяу_01__(&ctx).await { return Ok(()); }
    if let Err(_) = channel.edit(ctx.http(), poise::serenity_prelude::EditChannel::new().category(category)).await {
        crate::v2_components::мяу_v2_посылка_90__(ctx, serde_json::json!([{ "type": 17, "components": [{ "type": 10, "content": мяу!(&lang, "move_channel_error") }] }])).await?;
    } else {
        crate::v2_components::мяу_v2_посылка_90__(ctx, serde_json::json!([{ "type": 17, "components": [{ "type": 10, "content": мяу!(&lang, "move_channel_ok") }] }])).await?;
    }
    Ok(())
}

#[poise::command(slash_command, prefix_command, rename = "role")]
pub async fn мяяяяяу_10__(
    ctx: Context<'_>,
    #[description = "Role"] role: poise::serenity_prelude::RoleId,
    #[description = "User"] user: poise::serenity_prelude::Member,
) -> Result<(), Error> {
    let lang = мяяяяяу_18__(&ctx).await;
    if !мяяяяяу_01__(&ctx).await { return Ok(()); }
    if let Err(_) = user.add_role(ctx.http(), role).await {
        crate::v2_components::мяу_v2_посылка_90__(ctx, serde_json::json!([{ "type": 17, "components": [{ "type": 10, "content": мяу!(&lang, "assign_role_error") }] }])).await?;
    } else {
        crate::v2_components::мяу_v2_посылка_90__(ctx, serde_json::json!([{ "type": 17, "components": [{ "type": 10, "content": мяу!(&lang, "assign_role_ok") }] }])).await?;
    }
    Ok(())
}

#[poise::command(slash_command, prefix_command, rename = "vm")]
pub async fn мяяяяяу_11__(
    ctx: Context<'_>,
    #[description = "User"] user: poise::serenity_prelude::Member,
    #[description = "Voice Channel ID"] channel: poise::serenity_prelude::ChannelId,
) -> Result<(), Error> {
    let lang = мяяяяяу_18__(&ctx).await;
    if !мяяяяяу_01__(&ctx).await { return Ok(()); }
    let guild_id = ctx.guild_id().ok_or("No guild")?;
    if let Err(_) = guild_id.move_member(ctx.http(), user.user.id, channel).await {
        crate::v2_components::мяу_v2_посылка_90__(ctx, serde_json::json!([{ "type": 17, "components": [{ "type": 10, "content": мяу!(&lang, "move_member_error") }] }])).await?;
    } else {
        crate::v2_components::мяу_v2_посылка_90__(ctx, serde_json::json!([{ "type": 17, "components": [{ "type": 10, "content": мяу!(&lang, "move_member_ok") }] }])).await?;
    }
    Ok(())
}

#[poise::command(slash_command, prefix_command, rename = "flag")]
pub async fn мяяяяяу_12__(
    ctx: Context<'_>,
    #[description = "Action: add, rm"] action: String,
    #[description = "Channel ID"] channel_id: String,
    #[description = "Flag name"] flag_name: String,
) -> Result<(), Error> {
    let lang = мяяяяяу_18__(&ctx).await;
    if !мяяяяяу_01__(&ctx).await { return Ok(()); }
    let chan = channel_id.chars().filter(|c| c.is_ascii_digit()).collect::<String>();
    if chan.is_empty() {
        crate::v2_components::мяу_v2_посылка_90__(ctx, serde_json::json!([{ "type": 17, "components": [{ "type": 10, "content": мяу!(&lang, "invalid_channel_id") }] }])).await?;
        return Ok(());
    }
    let flag_name = flag_name.trim().to_lowercase();
    if flag_name.is_empty() {
        crate::v2_components::мяу_v2_посылка_90__(ctx, serde_json::json!([{ "type": 17, "components": [{ "type": 10, "content": мяу!(&lang, "empty_flag") }] }])).await?;
        return Ok(());
    }
    if action == "add" {
        {
            let mut flags = ctx.data().flags.write().await;
            let list = flags.entry(chan.clone()).or_insert_with(Vec::new);
            if !list.contains(&flag_name) {
                list.push(flag_name.clone());
            }
        }
        ctx.data().мяу_сохрани_flags_29__().await;
        crate::v2_components::мяу_v2_посылка_90__(ctx, serde_json::json!([{ "type": 17, "components": [{ "type": 10, "content": мяу!(&lang, "flag_added").replace("{flag}", &flag_name).replace("{channel}", &chan) }] }])).await?;
    } else if action == "rm" {
        {
            let mut flags = ctx.data().flags.write().await;
            let list = flags.entry(chan.clone()).or_insert_with(Vec::new);
            list.retain(|f| f != &flag_name);
        }
        ctx.data().мяу_сохрани_flags_29__().await;
        crate::v2_components::мяу_v2_посылка_90__(ctx, serde_json::json!([{ "type": 17, "components": [{ "type": 10, "content": мяу!(&lang, "flag_removed").replace("{flag}", &flag_name).replace("{channel}", &chan) }] }])).await?;
    } else {
        crate::v2_components::мяу_v2_посылка_90__(ctx, serde_json::json!([{ "type": 17, "components": [{ "type": 10, "content": мяу!(&lang, "invalid_action") }] }])).await?;
    }
    Ok(())
}

#[poise::command(slash_command, prefix_command, rename = "massrole")]
pub async fn мяяяяяу_13__(
    ctx: Context<'_>,
    #[description = "Role ID"] role_id: poise::serenity_prelude::RoleId,
) -> Result<(), Error> {
    let lang = мяяяяяу_18__(&ctx).await;
    if !мяяяяяу_01__(&ctx).await { return Ok(()); }
    let guild_id = ctx.guild_id().ok_or("No guild")?;
    crate::v2_components::мяу_v2_посылка_90__(ctx, serde_json::json!([{ "type": 17, "components": [{ "type": 10, "content": мяу!(&lang, "massrole_start") }] }])).await?;

    let mut success = 0;
    let mut fails = 0;

    let mut members = guild_id.members_iter(ctx.http()).boxed();
    use poise::futures_util::StreamExt;
    while let Some(member_res) = members.next().await {
        if let Ok(member) = member_res {
            if !member.user.bot && !member.roles.contains(&role_id) {
                if let Err(_) = member.add_role(ctx.http(), role_id).await {
                    fails += 1;
                } else {
                    success += 1;
                }
            }
        }
    }
    crate::v2_components::мяу_v2_посылка_90__(ctx, serde_json::json!([{ "type": 17, "components": [{ "type": 10, "content": мяу!(&lang, "massrole_done").replace("{ok}", &success.to_string()).replace("{fail}", &fails.to_string()) }] }])).await?;
    Ok(())
}

#[poise::command(slash_command, prefix_command, rename = "rrole")]
pub async fn мяяяяяу_14__(
    ctx: Context<'_>,
    #[description = "Role"] role: poise::serenity_prelude::RoleId,
    #[description = "Emoji"] emoji: String,
    #[description = "Channel"] channel: poise::serenity_prelude::ChannelId,
    #[description = "Message content"] content: Option<String>,
) -> Result<(), Error> {
    let lang = мяяяяяу_18__(&ctx).await;
    if !мяяяяяу_01__(&ctx).await { return Ok(()); }
    let text = content.unwrap_or_else(|| format!("React with {} to get the role with ID {}!", emoji, role));

    let msg = channel.say(ctx.http(), &text).await?;
    let reaction_type = poise::serenity_prelude::ReactionType::Unicode(emoji.clone());
    if let Err(e) = msg.react(ctx.http(), reaction_type).await {
        crate::v2_components::мяу_v2_посылка_90__(ctx, serde_json::json!([{ "type": 17, "components": [{ "type": 10, "content": мяу!(&lang, "rrole_react_fail").replace("{err}", &e.to_string()) }] }])).await?;
        return Ok(());
    }

    {
        let mut rr = ctx.data().reaction_roles.write().await;
        rr.insert(msg.id.to_string(), serde_json::json!({
            "roleId": role.to_string(),
            "emoji": emoji
        }));
    }
    ctx.data().мяу_сохрани_rroles_30__().await;

    crate::v2_components::мяу_v2_посылка_90__(ctx, serde_json::json!([{ "type": 17, "components": [{ "type": 10, "content": мяу!(&lang, "rrole_spawned").replace("{channel}", &channel.to_string()).replace("{role}", &role.to_string()) }] }])).await?;
    Ok(())
}

#[poise::command(slash_command, prefix_command, rename = "rtr")]
pub async fn мяяяяяу_15__(
    ctx: Context<'_>,
    #[description = "Old Role"] old_role: poise::serenity_prelude::RoleId,
    #[description = "New Role"] new_role: poise::serenity_prelude::RoleId,
) -> Result<(), Error> {
    let lang = мяяяяяу_18__(&ctx).await;
    if !мяяяяяу_01__(&ctx).await { return Ok(()); }
    let guild_id = ctx.guild_id().ok_or("No guild")?;
    crate::v2_components::мяу_v2_посылка_90__(ctx, serde_json::json!([{ "type": 17, "components": [{ "type": 10, "content": мяу!(&lang, "rtr_start") }] }])).await?;

    let mut success = 0;
    let mut fails = 0;

    let mut members = guild_id.members_iter(ctx.http()).boxed();
    use poise::futures_util::StreamExt;
    while let Some(member_res) = members.next().await {
        if let Ok(member) = member_res {
            if member.roles.contains(&old_role) {
                if let Err(_) = member.remove_role(ctx.http(), old_role).await {
                    fails += 1;
                    continue;
                }
                if let Err(_) = member.add_role(ctx.http(), new_role).await {
                    fails += 1;
                } else {
                    success += 1;
                }
            }
        }
    }

    crate::v2_components::мяу_v2_посылка_90__(ctx, serde_json::json!([{ "type": 17, "components": [{ "type": 10, "content": мяу!(&lang, "rtr_done").replace("{ok}", &success.to_string()).replace("{fail}", &fails.to_string()) }] }])).await?;
    Ok(())
}


#[poise::command(slash_command, prefix_command, rename = "help")]
pub async fn мяяяяяу_16__(
    ctx: Context<'_>,
) -> Result<(), Error> {
    if !мяяяяяу_01__(&ctx).await { return Ok(()); }
    let lang = мяяяяяу_18__(&ctx).await;
    crate::v2_components::мяу_v2_посылка_90__(ctx, мяяяяяу_23__(&lang, "help")).await?;
    Ok(())
}

fn мяяяяяу_17__(raw: &str) -> String {
    raw.chars().filter(|c| c.is_ascii_digit()).collect()
}

async fn мяяяяяу_18__(ctx: &Context<'_>) -> String {
    crate::i18n::мяу_язык_сервера_92__(ctx.data(), ctx.guild_id()).await
}

async fn мяяяяяу_19__(data: &МяуДанные, guild_id: Option<serenity::GuildId>) -> String {
    crate::i18n::мяу_язык_сервера_92__(data, guild_id).await
}

fn мяяяяяу_20__(lang: &str, key: &str) -> &'static str {
    match lang {
        "en" => match key {
            "unauthorized" => "[Terminal] Error: Unauthorized. Permission denied.",
            "empty" => "Empty",
            "wl_list" => "[Terminal] Whitelisted users:\n{list}",
            "wl_exists" => "[Terminal] Error: User is already whitelisted.",
            "wl_added" => "[Terminal] User added.",
            "wl_usage_add" => "[Terminal] Usage: wl add <id>",
            "wl_removed" => "[Terminal] User removed.",
            "wl_missing" => "[Terminal] Error: User is not whitelisted.",
            "wl_usage_rm" => "[Terminal] Usage: wl rm <id>",
            "wl_usage" => "[Terminal] Usage: wl <add|rm|list>",
            "invalid_channel_ids" => "[Terminal] Error: Invalid channel IDs.",
            "approval_mapped" => "[Terminal] Approval channel mapped.",
            "fetch_missing" => "[Terminal] Error: noorfetch.txt not found",
            "echo_error" => "[Terminal] Error formatting channel: {err}",
            "echo_ok" => "[Terminal] Channel topic updated.",
            "channel_created" => "[Terminal] Channel created.",
            "category_created" => "[Terminal] Category created.",
            "invalid_id" => "[Terminal] Invalid ID",
            "delete_message_error" => "[Terminal] Error deleting message",
            "delete_channel_error" => "[Terminal] Error deleting channel or category",
            "deleted" => "[Terminal] Deleted.",
            "rm_usage" => "[Terminal] Usage: rm <-c|-ch|-m> <id>",
            "move_channel_error" => "[Terminal] Error moving channel",
            "move_channel_ok" => "[Terminal] Channel moved.",
            "assign_role_error" => "[Terminal] Error assigning role",
            "assign_role_ok" => "[Terminal] Role assigned.",
            "move_member_error" => "[Terminal] Error moving member",
            "move_member_ok" => "[Terminal] Member moved.",
            "invalid_channel_id" => "[Terminal] Error: Invalid channel ID.",
            "empty_flag" => "[Terminal] Error: Flag name is empty.",
            "flag_added" => "[Terminal] Added flag '{flag}' to channel <#{channel}>",
            "flag_removed" => "[Terminal] Removed flag '{flag}' from channel <#{channel}>",
            "invalid_action" => "[Terminal] Invalid action.",
            "massrole_start" => "[Terminal] Assigning the role to all members. This may take time.",
            "massrole_done" => "[Terminal] Completed. Assigned: {ok}, Errors: {fail}.",
            "rrole_react_fail" => "[Terminal] Failed to add reaction: {err}",
            "rrole_spawned" => "[Terminal] Reaction role created in <#{channel}> for role ID {role}",
            "rtr_start" => "[Terminal] Role transfer may take some time.",
            "rtr_done" => "[Terminal] Completed. Swapped {ok} members. Errors: {fail}.",
            "docs_overview_label" => "Overview",
            "docs_ap_label" => "Approval Mapping",
            "docs_wl_label" => "Whitelist",
            "docs_fetch_label" => "Fetch",
            "docs_touch_label" => "Touch",
            "docs_mkdir_label" => "Mkdir",
            "docs_rm_label" => "Remove",
            "docs_mv_label" => "Move",
            "docs_vm_label" => "Voice Move",
            "docs_massrole_label" => "Mass Role",
            "docs_rrole_label" => "Reaction Role",
            "docs_rtr_label" => "Role Transfer",
            "docs_help" => "Terminal Control Panel\n\nSelect a command from the list below to view its syntax. For `flag`, you can also open an interactive configuration panel.",
            "docs_flag" => "Terminal Command: flag\n\nPurpose: configure a channel flag.\nSyntax: flag add <channel> <flag>\nSyntax: flag rm <channel> <flag>\nInteractive mode: run `flag` in the terminal channel and use the panel.",
            "docs_ap" => "Terminal Command: ap\n\nPurpose: map a request channel to an approval channel.\nSyntax: ap <request_channel> <approval_channel>",
            "docs_wl" => "Terminal Command: wl\n\nPurpose: manage the terminal whitelist.\nSyntax: wl list\nSyntax: wl add <user>\nSyntax: wl rm <user>",
            "docs_fetch" => "Terminal Command: fetch\n\nPurpose: show the system information banner.\nSyntax: fetch",
            "docs_echo" => "Terminal Command: echo\n\nPurpose: update a channel topic.\nSyntax: echo <channel> <description>",
            "docs_touch" => "Terminal Command: touch\n\nPurpose: create a text channel.\nSyntax: touch <name> [category]",
            "docs_mkdir" => "Terminal Command: mkdir\n\nPurpose: create a category.\nSyntax: mkdir <name>",
            "docs_rm" => "Terminal Command: rm\n\nPurpose: remove a message, channel, or category.\nSyntax: rm -m <message_id>\nSyntax: rm -ch <channel_id>\nSyntax: rm -c <category_id>",
            "docs_mv" => "Terminal Command: mv\n\nPurpose: move a channel into a category.\nSyntax: mv <channel> <category>",
            "docs_role" => "Terminal Command: role\n\nPurpose: assign a role to a member.\nSyntax: role <role> <user>",
            "docs_vm" => "Terminal Command: vm\n\nPurpose: move a member into a voice channel.\nSyntax: vm <user> <voice_channel>",
            "docs_massrole" => "Terminal Command: massrole\n\nPurpose: assign a role to every non-bot member.\nSyntax: massrole <role>",
            "docs_rrole" => "Terminal Command: rrole\n\nPurpose: create a reaction-role message.\nSyntax: rrole <role> <emoji> <channel> [message]",
            "docs_rtr" => "Terminal Command: rtr\n\nPurpose: replace one role with another across the guild.\nSyntax: rtr <old_role> <new_role>",
            "select_command" => "Select a terminal command",
            "open_flag_panel" => "Open Flag Panel",
            "close" => "Close",
            "flag_title" => "Flag Configuration",
            "not_selected" => "Not selected",
            "flag_channel" => "Channel",
            "flag_action" => "Action",
            "flag_name" => "Flag",
            "flag_available" => "Available flags in the interactive panel: request",
            "flag_status" => "Status: {status}",
            "select_target_channel" => "Select a target channel",
            "select_action" => "Select an action",
            "add" => "Add",
            "remove" => "Remove",
            "select_flag" => "Select a flag",
            "apply" => "Apply",
            "panel_other_operator" => "This panel belongs to another operator.",
            "select_channel_first" => "Select a channel first.",
            "select_action_first" => "Select an action first.",
            "select_flag_first" => "Select a flag first.",
            "flag_apply_added" => "Flag '{flag}' was added to <#{channel}>.",
            "flag_apply_removed" => "Flag '{flag}' was removed from <#{channel}>.",
            "unknown_command" => "[Terminal] Unknown command. Use: help",
            "usage_ap_msg" => "[Terminal] Usage: ap <request_channel_id> <approval_channel_id>",
            "usage_wl_add_msg" => "[Terminal] Usage: wl add <@user_or_id>",
            "usage_wl_rm_msg" => "[Terminal] Usage: wl rm <@user_or_id>",
            "usage_wl_msg" => "[Terminal] Usage: wl <add|rm|list> [@user_or_id]",
            "usage_echo_msg" => "[Terminal] Usage: echo <channel_id_or_mention> <description>",
            "channel_not_found" => "[Terminal] Error: Channel not found.",
            "usage_touch_msg" => "[Terminal] Usage: touch <channel_name> [category_id]",
            "usage_mkdir_msg" => "[Terminal] Usage: mkdir <category_name>",
            "usage_mv_msg" => "[Terminal] Usage: mv <channel> <category>",
            "invalid_ids" => "[Terminal] Error: Invalid IDs",
            "usage_role_msg" => "[Terminal] Usage: role <role_id_or_mention> <user_id_or_mention>",
            "usage_vm_msg" => "[Terminal] Usage: vm <user_id> <voice_channel_id>",
            "usage_flag_msg" => "[Terminal] Usage: flag add|rm <channel> <flag>",
            "usage_massrole_msg" => "[Terminal] Usage: massrole <role_id_or_mention>",
            "invalid_role" => "[Terminal] Error: Invalid role",
            "usage_rrole_msg" => "[Terminal] Usage: rrole @role <emoji> #channel [message]",
            "invalid_arguments" => "[Terminal] Error: Invalid arguments.",
            "usage_rtr_msg" => "[Terminal] Usage: rtr <@old_role> <@new_role>",
            "invalid_role_input" => "[Terminal] Error: Invalid role input.",
            _ => "",
        },
        _ => match key {
            "unauthorized" => "[Terminal] Ошибка: недостаточно прав.",
            "empty" => "Пусто",
            "wl_list" => "[Terminal] Пользователи в whitelist:\n{list}",
            "wl_exists" => "[Terminal] Ошибка: пользователь уже в whitelist.",
            "wl_added" => "[Terminal] Пользователь добавлен.",
            "wl_usage_add" => "[Terminal] Использование: wl add <id>",
            "wl_removed" => "[Terminal] Пользователь удален.",
            "wl_missing" => "[Terminal] Ошибка: пользователя нет в whitelist.",
            "wl_usage_rm" => "[Terminal] Использование: wl rm <id>",
            "wl_usage" => "[Terminal] Использование: wl <add|rm|list>",
            "invalid_channel_ids" => "[Terminal] Ошибка: неверные ID каналов.",
            "approval_mapped" => "[Terminal] Канал подтверждения сохранен.",
            "fetch_missing" => "[Terminal] Ошибка: файл noorfetch.txt не найден",
            "echo_error" => "[Terminal] Ошибка изменения темы канала: {err}",
            "echo_ok" => "[Terminal] Тема канала обновлена.",
            "channel_created" => "[Terminal] Канал создан.",
            "category_created" => "[Terminal] Категория создана.",
            "invalid_id" => "[Terminal] Неверный ID",
            "delete_message_error" => "[Terminal] Ошибка удаления сообщения",
            "delete_channel_error" => "[Terminal] Ошибка удаления канала или категории",
            "deleted" => "[Terminal] Удалено.",
            "rm_usage" => "[Terminal] Использование: rm <-c|-ch|-m> <id>",
            "move_channel_error" => "[Terminal] Ошибка перемещения канала",
            "move_channel_ok" => "[Terminal] Канал перемещен.",
            "assign_role_error" => "[Terminal] Ошибка выдачи роли",
            "assign_role_ok" => "[Terminal] Роль выдана.",
            "move_member_error" => "[Terminal] Ошибка перемещения участника",
            "move_member_ok" => "[Terminal] Участник перемещен.",
            "invalid_channel_id" => "[Terminal] Ошибка: неверный ID канала.",
            "empty_flag" => "[Terminal] Ошибка: имя флага пустое.",
            "flag_added" => "[Terminal] Флаг '{flag}' добавлен каналу <#{channel}>",
            "flag_removed" => "[Terminal] Флаг '{flag}' удален у канала <#{channel}>",
            "invalid_action" => "[Terminal] Неверное действие.",
            "massrole_start" => "[Terminal] Выдача роли всем участникам. Это может занять время.",
            "massrole_done" => "[Terminal] Готово. Выдано: {ok}, Ошибок: {fail}.",
            "rrole_react_fail" => "[Terminal] Не удалось добавить реакцию: {err}",
            "rrole_spawned" => "[Terminal] Сообщение reaction role создано в <#{channel}> для роли ID {role}",
            "rtr_start" => "[Terminal] Перенос роли может занять некоторое время.",
            "rtr_done" => "[Terminal] Готово. Роль заменена у {ok} участников. Ошибок: {fail}.",
            "docs_overview_label" => "Обзор",
            "docs_ap_label" => "Канал подтверждения",
            "docs_wl_label" => "Whitelist",
            "docs_fetch_label" => "Fetch",
            "docs_touch_label" => "Создать канал",
            "docs_mkdir_label" => "Создать категорию",
            "docs_rm_label" => "Удаление",
            "docs_mv_label" => "Перемещение",
            "docs_vm_label" => "Voice Move",
            "docs_massrole_label" => "Массовая роль",
            "docs_rrole_label" => "Reaction Role",
            "docs_rtr_label" => "Перенос роли",
            "docs_help" => "Панель терминала\n\nВыбери команду из списка ниже, чтобы посмотреть синтаксис. Для `flag` также доступна интерактивная панель настройки.",
            "docs_flag" => "Команда терминала: flag\n\nНазначение: настраивает флаг канала.\nСинтаксис: flag add <channel> <flag>\nСинтаксис: flag rm <channel> <flag>\nИнтерактивный режим: напиши `flag` в канале терминала и используй панель.",
            "docs_ap" => "Команда терминала: ap\n\nНазначение: связывает request-канал с каналом подтверждения.\nСинтаксис: ap <request_channel> <approval_channel>",
            "docs_wl" => "Команда терминала: wl\n\nНазначение: управляет whitelist терминала.\nСинтаксис: wl list\nСинтаксис: wl add <user>\nСинтаксис: wl rm <user>",
            "docs_fetch" => "Команда терминала: fetch\n\nНазначение: показывает системный баннер.\nСинтаксис: fetch",
            "docs_echo" => "Команда терминала: echo\n\nНазначение: меняет тему канала.\nСинтаксис: echo <channel> <description>",
            "docs_touch" => "Команда терминала: touch\n\nНазначение: создает текстовый канал.\nСинтаксис: touch <name> [category]",
            "docs_mkdir" => "Команда терминала: mkdir\n\nНазначение: создает категорию.\nСинтаксис: mkdir <name>",
            "docs_rm" => "Команда терминала: rm\n\nНазначение: удаляет сообщение, канал или категорию.\nСинтаксис: rm -m <message_id>\nСинтаксис: rm -ch <channel_id>\nСинтаксис: rm -c <category_id>",
            "docs_mv" => "Команда терминала: mv\n\nНазначение: перемещает канал в категорию.\nСинтаксис: mv <channel> <category>",
            "docs_role" => "Команда терминала: role\n\nНазначение: выдает роль участнику.\nСинтаксис: role <role> <user>",
            "docs_vm" => "Команда терминала: vm\n\nНазначение: перемещает участника в голосовой канал.\nСинтаксис: vm <user> <voice_channel>",
            "docs_massrole" => "Команда терминала: massrole\n\nНазначение: выдает роль всем участникам без ботов.\nСинтаксис: massrole <role>",
            "docs_rrole" => "Команда терминала: rrole\n\nНазначение: создает сообщение reaction-role.\nСинтаксис: rrole <role> <emoji> <channel> [message]",
            "docs_rtr" => "Команда терминала: rtr\n\nНазначение: заменяет одну роль на другую по всему серверу.\nСинтаксис: rtr <old_role> <new_role>",
            "select_command" => "Выбери команду терминала",
            "open_flag_panel" => "Открыть панель flag",
            "close" => "Закрыть",
            "flag_title" => "Настройка флага",
            "not_selected" => "Не выбрано",
            "flag_channel" => "Канал",
            "flag_action" => "Действие",
            "flag_name" => "Флаг",
            "flag_available" => "Доступные флаги в интерактивной панели: request",
            "flag_status" => "Статус: {status}",
            "select_target_channel" => "Выбери целевой канал",
            "select_action" => "Выбери действие",
            "add" => "Добавить",
            "remove" => "Удалить",
            "select_flag" => "Выбери флаг",
            "apply" => "Применить",
            "panel_other_operator" => "Эта панель принадлежит другому оператору.",
            "select_channel_first" => "Сначала выбери канал.",
            "select_action_first" => "Сначала выбери действие.",
            "select_flag_first" => "Сначала выбери флаг.",
            "flag_apply_added" => "Флаг '{flag}' добавлен каналу <#{channel}>.",
            "flag_apply_removed" => "Флаг '{flag}' удален у канала <#{channel}>.",
            "unknown_command" => "[Terminal] Неизвестная команда. Используй: help",
            "usage_ap_msg" => "[Terminal] Использование: ap <request_channel_id> <approval_channel_id>",
            "usage_wl_add_msg" => "[Terminal] Использование: wl add <@user_or_id>",
            "usage_wl_rm_msg" => "[Terminal] Использование: wl rm <@user_or_id>",
            "usage_wl_msg" => "[Terminal] Использование: wl <add|rm|list> [@user_or_id]",
            "usage_echo_msg" => "[Terminal] Использование: echo <channel_id_or_mention> <description>",
            "channel_not_found" => "[Terminal] Ошибка: канал не найден.",
            "usage_touch_msg" => "[Terminal] Использование: touch <channel_name> [category_id]",
            "usage_mkdir_msg" => "[Terminal] Использование: mkdir <category_name>",
            "usage_mv_msg" => "[Terminal] Использование: mv <channel> <category>",
            "invalid_ids" => "[Terminal] Ошибка: неверные ID",
            "usage_role_msg" => "[Terminal] Использование: role <role_id_or_mention> <user_id_or_mention>",
            "usage_vm_msg" => "[Terminal] Использование: vm <user_id> <voice_channel_id>",
            "usage_flag_msg" => "[Terminal] Использование: flag add|rm <channel> <flag>",
            "usage_massrole_msg" => "[Terminal] Использование: massrole <role_id_or_mention>",
            "invalid_role" => "[Terminal] Ошибка: неверная роль",
            "usage_rrole_msg" => "[Terminal] Использование: rrole @role <emoji> #channel [message]",
            "invalid_arguments" => "[Terminal] Ошибка: неверные аргументы.",
            "usage_rtr_msg" => "[Terminal] Использование: rtr <@old_role> <@new_role>",
            "invalid_role_input" => "[Terminal] Ошибка: неверный ввод роли.",
            _ => "",
        },
    }
}

fn мяяяяяу_21__(lang: &str) -> Vec<serde_json::Value> {
    [
        ("help", мяу!(lang, "docs_overview_label")),
        ("flag", "Flag"),
        ("ap", мяу!(lang, "docs_ap_label")),
        ("wl", мяу!(lang, "docs_wl_label")),
        ("fetch", мяу!(lang, "docs_fetch_label")),
        ("echo", "Echo"),
        ("touch", мяу!(lang, "docs_touch_label")),
        ("mkdir", мяу!(lang, "docs_mkdir_label")),
        ("rm", мяу!(lang, "docs_rm_label")),
        ("mv", мяу!(lang, "docs_mv_label")),
        ("role", "Role"),
        ("vm", мяу!(lang, "docs_vm_label")),
        ("massrole", мяу!(lang, "docs_massrole_label")),
        ("rrole", мяу!(lang, "docs_rrole_label")),
        ("rtr", мяу!(lang, "docs_rtr_label")),
    ]
    .into_iter()
    .map(|(value, label)| json!({ "label": label, "value": value }))
    .collect()
}

fn мяяяяяу_22__(lang: &str, command: &str) -> String {
    match command {
        "flag" => мяу!(lang, "docs_flag").to_string(),
        "ap" => мяу!(lang, "docs_ap").to_string(),
        "wl" => мяу!(lang, "docs_wl").to_string(),
        "fetch" => мяу!(lang, "docs_fetch").to_string(),
        "echo" => мяу!(lang, "docs_echo").to_string(),
        "touch" => мяу!(lang, "docs_touch").to_string(),
        "mkdir" => мяу!(lang, "docs_mkdir").to_string(),
        "rm" => мяу!(lang, "docs_rm").to_string(),
        "mv" => мяу!(lang, "docs_mv").to_string(),
        "role" => мяу!(lang, "docs_role").to_string(),
        "vm" => мяу!(lang, "docs_vm").to_string(),
        "massrole" => мяу!(lang, "docs_massrole").to_string(),
        "rrole" => мяу!(lang, "docs_rrole").to_string(),
        "rtr" => мяу!(lang, "docs_rtr").to_string(),
        _ => мяу!(lang, "docs_help").to_string(),
    }
}

fn мяяяяяу_23__(lang: &str, selected: &str) -> serde_json::Value {
    json!([
        {
            "type": 17,
            "components": [
                {
                    "type": 10,
                    "content": мяяяяяу_22__(lang, selected)
                }
            ]
        },
        {
            "type": 1,
            "components": [
                {
                    "type": 3,
                    "custom_id": "terminal_docs_select",
                    "placeholder": мяу!(lang, "select_command"),
                    "options": мяяяяяу_21__(lang)
                }
            ]
        },
        {
            "type": 1,
            "components": [
                {
                    "type": 2,
                    "style": 2,
                    "label": мяу!(lang, "open_flag_panel"),
                    "custom_id": "terminal_flag_open"
                },
                {
                    "type": 2,
                    "style": 2,
                    "label": мяу!(lang, "close"),
                    "custom_id": "terminal_docs_close"
                }
            ]
        }
    ])
}

fn мяяяяяу_24__(lang: &str, draft: &МяуЧерновикФлага, status: Option<&str>) -> String {
    let channel = draft
        .target_channel_id
        .as_ref()
        .map(|id| format!("<#{}>", id))
        .unwrap_or_else(|| мяу!(lang, "not_selected").to_string());
    let action = draft.action.clone().unwrap_or_else(|| мяу!(lang, "not_selected").to_string());
    let flag = draft.flag.clone().unwrap_or_else(|| мяу!(lang, "not_selected").to_string());
    let mut text = format!(
        "{}\n\n{}: {}\n{}: {}\n{}: {}\n\n{}",
        мяу!(lang, "flag_title"),
        мяу!(lang, "flag_channel"), channel,
        мяу!(lang, "flag_action"), action,
        мяу!(lang, "flag_name"), flag,
        мяу!(lang, "flag_available")
    );
    if let Some(status) = status {
        text.push_str(&format!(
            "\n\n{}",
            мяу!(lang, "flag_status").replace("{status}", status)
        ));
    }
    text
}

fn мяяяяяу_25__(lang: &str, draft: &МяуЧерновикФлага, status: Option<&str>) -> serde_json::Value {
    let ready = draft.target_channel_id.is_some() && draft.action.is_some() && draft.flag.is_some();
    json!([
        {
            "type": 17,
            "components": [
                {
                    "type": 10,
                    "content": мяяяяяу_24__(lang, draft, status)
                }
            ]
        },
        {
            "type": 1,
            "components": [
                {
                    "type": 8,
                    "custom_id": "terminal_flag_channel",
                    "placeholder": мяу!(lang, "select_target_channel"),
                    "channel_types": [0]
                }
            ]
        },
        {
            "type": 1,
            "components": [
                {
                    "type": 3,
                    "custom_id": "terminal_flag_action",
                    "placeholder": мяу!(lang, "select_action"),
                    "options": [
                        { "label": мяу!(lang, "add"), "value": "add" },
                        { "label": мяу!(lang, "remove"), "value": "rm" }
                    ]
                }
            ]
        },
        {
            "type": 1,
            "components": [
                {
                    "type": 3,
                    "custom_id": "terminal_flag_name",
                    "placeholder": мяу!(lang, "select_flag"),
                    "options": [
                        { "label": "request", "value": "request" }
                    ]
                }
            ]
        },
        {
            "type": 1,
            "components": [
                {
                    "type": 2,
                    "style": 1,
                    "label": мяу!(lang, "apply"),
                    "custom_id": "terminal_flag_apply",
                    "disabled": !ready
                },
                {
                    "type": 2,
                    "style": 2,
                    "label": мяу!(lang, "close"),
                    "custom_id": "terminal_flag_close"
                }
            ]
        }
    ])
}

async fn мяяяяяу_26__(
    ctx: &serenity::Context,
    channel_id: serenity::ChannelId,
    components: serde_json::Value,
) -> Result<serenity::Message, Error> {
    Ok(ctx
        .http
        .send_message(
            channel_id,
            vec![],
            &json!({
                "flags": 1 << 15,
                "components": components
            }),
        )
        .await?)
}

async fn мяяяяяу_27__(
    ctx: &serenity::Context,
    component: &serenity::ComponentInteraction,
    components: serde_json::Value,
) -> Result<(), Error> {
    ctx.http.create_interaction_response(
        component.id,
        &component.token,
        &json!({
            "type": 7,
            "data": {
                "flags": 1 << 15,
                "components": components
            }
        }),
        vec![],
    ).await?;
    Ok(())
}

async fn мяяяяяу_28__(ctx: &serenity::Context, data: &МяуДанные, message: &serenity::Message) -> bool {
    let uid = message.author.id.to_string();
    if let Some(guild_id) = message.guild_id {
        if let Some(guild) = ctx.cache.guild(guild_id) {
            if guild.owner_id == message.author.id {
                return true;
            }
        }
        if let Ok(member) = guild_id.member(&ctx.http, message.author.id).await {
            if let Ok(perms) = member.permissions(&ctx.cache) {
                if perms.administrator() {
                    return true;
                }
            }
            if let Some(role_id) = data.мяу_роль_стаффа_39__(Some(guild_id)).await {
                if member.roles.contains(&role_id) {
                    return true;
                }
            }
        }
    }
    let wl = data.terminal_whitelist.read().await;
    if wl.contains(&uid) {
        return true;
    }
    false
}

pub async fn мяяяяяу_29__(
    ctx: &serenity::Context,
    data: &МяуДанные,
    component: &serenity::ComponentInteraction,
) -> Result<bool, Error> {
    let custom_id = component.data.custom_id.as_str();
    if !custom_id.starts_with("terminal_") {
        return Ok(false);
    }

    let user_id = component.user.id.to_string();
    let lang = мяяяяяу_19__(data, component.guild_id).await;

    match custom_id {
        "terminal_docs_select" => {
            if let serenity::ComponentInteractionDataKind::StringSelect { values } = &component.data.kind {
                let selected = values.first().map(|value| value.as_str()).unwrap_or("help");
                мяяяяяу_27__(ctx, component, мяяяяяу_23__(&lang, selected)).await?;
            } else {
                component.defer(ctx).await?;
            }
            Ok(true)
        }
        "terminal_docs_close" | "terminal_flag_close" => {
            component.defer(ctx).await?;
            let _ = component.message.delete(ctx).await;
            if custom_id == "terminal_flag_close" {
                data.terminal_flag_drafts.write().await.remove(&component.message.id.to_string());
            }
            Ok(true)
        }
        "terminal_flag_open" => {
            let draft = МяуЧерновикФлага {
                owner_user_id: user_id,
                ..Default::default()
            };
            data.terminal_flag_drafts
                .write()
                .await
                .insert(component.message.id.to_string(), draft.clone());
            мяяяяяу_27__(ctx, component, мяяяяяу_25__(&lang, &draft, None)).await?;
            Ok(true)
        }
        "terminal_flag_channel" | "terminal_flag_action" | "terminal_flag_name" | "terminal_flag_apply" => {
            let message_key = component.message.id.to_string();
            let mut drafts = data.terminal_flag_drafts.write().await;
            let draft = drafts.entry(message_key.clone()).or_insert_with(|| МяуЧерновикФлага {
                owner_user_id: user_id.clone(),
                ..Default::default()
            });

            if draft.owner_user_id != user_id {
                component.create_response(
                    ctx,
                        serenity::CreateInteractionResponse::Message(
                            serenity::CreateInteractionResponseMessage::new()
                            .content(мяу!(&lang, "panel_other_operator"))
                            .ephemeral(true),
                    ),
                ).await?;
                return Ok(true);
            }

            match custom_id {
                "terminal_flag_channel" => {
                    if let serenity::ComponentInteractionDataKind::ChannelSelect { values } = &component.data.kind {
                        draft.target_channel_id = values.first().map(|value| value.to_string());
                    }
                    let next = draft.clone();
                    drop(drafts);
                    мяяяяяу_27__(ctx, component, мяяяяяу_25__(&lang, &next, None)).await?;
                }
                "terminal_flag_action" => {
                    if let serenity::ComponentInteractionDataKind::StringSelect { values } = &component.data.kind {
                        draft.action = values.first().cloned();
                    }
                    let next = draft.clone();
                    drop(drafts);
                    мяяяяяу_27__(ctx, component, мяяяяяу_25__(&lang, &next, None)).await?;
                }
                "terminal_flag_name" => {
                    if let serenity::ComponentInteractionDataKind::StringSelect { values } = &component.data.kind {
                        draft.flag = values.first().cloned();
                    }
                    let next = draft.clone();
                    drop(drafts);
                    мяяяяяу_27__(ctx, component, мяяяяяу_25__(&lang, &next, None)).await?;
                }
                "terminal_flag_apply" => {
                    let target_channel_id = draft.target_channel_id.clone();
                    let action = draft.action.clone();
                    let flag = draft.flag.clone();
                    drop(drafts);

                    let Some(target_channel_id) = target_channel_id else {
                        мяяяяяу_27__(
                            ctx,
                            component,
                            мяяяяяу_25__(&lang, &МяуЧерновикФлага::default(), Some(мяу!(&lang, "select_channel_first"))),
                        ).await?;
                        return Ok(true);
                    };
                    let Some(action) = action else {
                        мяяяяяу_27__(
                            ctx,
                            component,
                            мяяяяяу_25__(&lang, &МяуЧерновикФлага::default(), Some(мяу!(&lang, "select_action_first"))),
                        ).await?;
                        return Ok(true);
                    };
                    let Some(flag) = flag else {
                        мяяяяяу_27__(
                            ctx,
                            component,
                            мяяяяяу_25__(&lang, &МяуЧерновикФлага::default(), Some(мяу!(&lang, "select_flag_first"))),
                        ).await?;
                        return Ok(true);
                    };

                    let current_draft = МяуЧерновикФлага {
                        owner_user_id: user_id,
                        target_channel_id: Some(target_channel_id.clone()),
                        action: Some(action.clone()),
                        flag: Some(flag.clone()),
                    };

                    {
                        let mut flags = data.flags.write().await;
                        let list = flags.entry(target_channel_id.clone()).or_default();
                        if action == "add" {
                            if !list.contains(&flag) {
                                list.push(flag.clone());
                            }
                        } else {
                            list.retain(|entry| entry != &flag);
                        }
                    }
                    data.мяу_сохрани_flags_29__().await;
                    data.terminal_flag_drafts
                        .write()
                        .await
                        .insert(component.message.id.to_string(), current_draft.clone());
                    let status = if action == "add" {
                        мяу!(&lang, "flag_apply_added")
                            .replace("{flag}", &flag)
                            .replace("{channel}", &target_channel_id)
                    } else {
                        мяу!(&lang, "flag_apply_removed")
                            .replace("{flag}", &flag)
                            .replace("{channel}", &target_channel_id)
                    };
                    мяяяяяу_27__(ctx, component, мяяяяяу_25__(&lang, &current_draft, Some(&status))).await?;
                }
                _ => {}
            }

            Ok(true)
        }
        _ => Ok(false),
    }
}

pub async fn мяяяяяу_30__(
    ctx: &serenity::Context,
    data: &МяуДанные,
    message: &serenity::Message,
) -> Result<bool, Error> {
    let terminal_channel_id = data
        .мяу_конфиг_сервера_38__(message.guild_id)
        .await
        .terminal_channel_id
        .and_then(|id| id.parse::<u64>().ok());
    if terminal_channel_id.map(serenity::ChannelId::new) != Some(message.channel_id) {
        return Ok(false);
    }
    let parts: Vec<&str> = message.content.split_whitespace().collect();
    if parts.is_empty() {
        return Ok(true);
    }
    let lang = мяяяяяу_19__(data, message.guild_id).await;
    if !мяяяяяу_28__(ctx, data, message).await {
        message
            .channel_id
            .say(ctx, мяу!(&lang, "unauthorized"))
            .await?;
        return Ok(true);
    }

    let guild_id = match message.guild_id {
        Some(g) => g,
        None => return Ok(true),
    };
    let cmd = parts[0].to_lowercase();

    match cmd.as_str() {
        "wl" => {
            let action = parts.get(1).map(|s| s.to_lowercase()).unwrap_or_default();
            match action.as_str() {
                "list" => {
                    let wl = data.terminal_whitelist.read().await;
                    let text = if wl.is_empty() {
                        мяу!(&lang, "empty").to_string()
                    } else {
                        wl.iter().map(|id| format!("<@{}>", id)).collect::<Vec<_>>().join(", ")
                    };
                    ctx.http.send_message(message.channel_id, vec![], &serde_json::json!({ "flags": 1<<15, "components": [{ "type": 17, "components": [{ "type": 10, "content": мяу!(&lang, "wl_list").replace("{list}", &text) }] }] })).await.ok();
                }
                "add" => {
                    let target = parts.get(2).map(|s| мяяяяяу_17__(s)).unwrap_or_default();
                    if target.is_empty() {
                        ctx.http.send_message(message.channel_id, vec![], &serde_json::json!({ "flags": 1<<15, "components": [{ "type": 17, "components": [{ "type": 10, "content": мяу!(&lang, "usage_wl_add_msg") }] }] })).await.ok();
                    } else {
                        let added = {
                            let mut wl = data.terminal_whitelist.write().await;
                            if wl.contains(&target) {
                                false
                            } else {
                                wl.push(target);
                                true
                            }
                        };
                        if !added {
                            ctx.http.send_message(message.channel_id, vec![], &serde_json::json!({ "flags": 1<<15, "components": [{ "type": 17, "components": [{ "type": 10, "content": мяу!(&lang, "wl_exists") }] }] })).await.ok();
                        } else {
                            data.мяу_сохрани_whitelist_27__().await;
                            let _ = message.react(ctx, serenity::ReactionType::Unicode("".into())).await;
                        }
                    }
                }
                "rm" => {
                    let target = parts.get(2).map(|s| мяяяяяу_17__(s)).unwrap_or_default();
                    if target.is_empty() {
                        ctx.http.send_message(message.channel_id, vec![], &serde_json::json!({ "flags": 1<<15, "components": [{ "type": 17, "components": [{ "type": 10, "content": мяу!(&lang, "usage_wl_rm_msg") }] }] })).await.ok();
                    } else {
                        let removed = {
                            let mut wl = data.terminal_whitelist.write().await;
                            if wl.contains(&target) {
                                wl.retain(|x| x != &target);
                                true
                            } else {
                                false
                            }
                        };
                        if !removed {
                            ctx.http.send_message(message.channel_id, vec![], &serde_json::json!({ "flags": 1<<15, "components": [{ "type": 17, "components": [{ "type": 10, "content": мяу!(&lang, "wl_missing") }] }] })).await.ok();
                        } else {
                            data.мяу_сохрани_whitelist_27__().await;
                            let _ = message.react(ctx, serenity::ReactionType::Unicode("".into())).await;
                        }
                    }
                }
                _ => {
                    ctx.http.send_message(message.channel_id, vec![], &serde_json::json!({ "flags": 1<<15, "components": [{ "type": 17, "components": [{ "type": 10, "content": мяу!(&lang, "usage_wl_msg") }] }] })).await.ok();
                }
            }
            return Ok(true);
        }
        "ap" => {
            if parts.len() < 3 {
                мяяяяяу_26__(ctx, message.channel_id, мяяяяяу_23__(&lang, "ap")).await.ok();
                return Ok(true);
            }
            let req = мяяяяяу_17__(parts[1]);
            let app = мяяяяяу_17__(parts[2]);
            if req.is_empty() || app.is_empty() {
                ctx.http.send_message(message.channel_id, vec![], &serde_json::json!({ "flags": 1<<15, "components": [{ "type": 17, "components": [{ "type": 10, "content": мяу!(&lang, "invalid_channel_ids") }] }] })).await.ok();
                return Ok(true);
            }
            {
                let mut channels = data.approval_channels.write().await;
                channels.insert(req, app);
            }
            data.мяу_сохрани_approval_28__().await;
            let _ = message.react(ctx, serenity::ReactionType::Unicode("".into())).await;
            return Ok(true);
        }
        "fetch" => {
            let path = std::path::PathBuf::from("../assets/noorfetch.txt");
            if let Ok(text) = std::fs::read_to_string(path) {
                ctx.http.send_message(message.channel_id, vec![], &serde_json::json!({ "flags": 1<<15, "components": [{ "type": 17, "components": [{ "type": 10, "content": format!("```\n{}\n```", text) }] }] })).await.ok();
            } else {
                ctx.http.send_message(message.channel_id, vec![], &serde_json::json!({ "flags": 1<<15, "components": [{ "type": 17, "components": [{ "type": 10, "content": мяу!(&lang, "fetch_missing") }] }] })).await.ok();
            }
            return Ok(true);
        }
        "echo" => {
            if parts.len() < 3 {
                ctx.http.send_message(message.channel_id, vec![], &serde_json::json!({ "flags": 1<<15, "components": [{ "type": 17, "components": [{ "type": 10, "content": мяу!(&lang, "usage_echo_msg") }] }] })).await.ok();
                return Ok(true);
            }
            let channel_id = мяяяяяу_17__(parts[1]).parse::<u64>().unwrap_or(0);
            if channel_id == 0 {
                ctx.http.send_message(message.channel_id, vec![], &serde_json::json!({ "flags": 1<<15, "components": [{ "type": 17, "components": [{ "type": 10, "content": мяу!(&lang, "channel_not_found") }] }] })).await.ok();
                return Ok(true);
            }
            let description = parts[2..].join(" ");
            let mut channel = match serenity::ChannelId::new(channel_id).to_channel(ctx).await {
                Ok(serenity::Channel::Guild(ch)) => ch,
                _ => {
                    ctx.http.send_message(message.channel_id, vec![], &serde_json::json!({ "flags": 1<<15, "components": [{ "type": 17, "components": [{ "type": 10, "content": мяу!(&lang, "channel_not_found") }] }] })).await.ok();
                    return Ok(true);
                }
            };
            if channel
                .edit(ctx, serenity::EditChannel::new().topic(description))
                .await
                .is_ok()
            {
                let _ = message.react(ctx, serenity::ReactionType::Unicode("".into())).await;
            } else {
                ctx.http.send_message(message.channel_id, vec![], &serde_json::json!({ "flags": 1<<15, "components": [{ "type": 17, "components": [{ "type": 10, "content": мяу!(&lang, "echo_error").replace("{err}", "") }] }] })).await.ok();
            }
            return Ok(true);
        }
        "touch" => {
            if parts.len() < 2 {
                ctx.http.send_message(message.channel_id, vec![], &serde_json::json!({ "flags": 1<<15, "components": [{ "type": 17, "components": [{ "type": 10, "content": "`[Terminal] Usage: touch <channel_name> [category_id]`" }] }] })).await.ok();
                return Ok(true);
            }
            let mut builder = serenity::CreateChannel::new(parts[1].to_string()).kind(serenity::ChannelType::Text);
            if let Some(cat_raw) = parts.get(2) {
                let cat = мяяяяяу_17__(cat_raw).parse::<u64>().unwrap_or(0);
                if cat != 0 {
                    builder = builder.category(serenity::ChannelId::new(cat));
                }
            }
            if guild_id.create_channel(ctx, builder).await.is_ok() {
                let _ = message.react(ctx, serenity::ReactionType::Unicode("".into())).await;
            }
            return Ok(true);
        }
        "mkdir" => {
            if parts.len() < 2 {
                ctx.http.send_message(message.channel_id, vec![], &serde_json::json!({ "flags": 1<<15, "components": [{ "type": 17, "components": [{ "type": 10, "content": "`[Terminal] Usage: mkdir <category_name>`" }] }] })).await.ok();
                return Ok(true);
            }
            let builder = serenity::CreateChannel::new(parts[1].to_string()).kind(serenity::ChannelType::Category);
            if guild_id.create_channel(ctx, builder).await.is_ok() {
                let _ = message.react(ctx, serenity::ReactionType::Unicode("".into())).await;
            }
            return Ok(true);
        }
        "rm" => {
            if parts.len() < 3 {
                ctx.http.send_message(message.channel_id, vec![], &serde_json::json!({ "flags": 1<<15, "components": [{ "type": 17, "components": [{ "type": 10, "content": "`[Terminal] Usage: rm -c|-ch|-m <id>`" }] }] })).await.ok();
                return Ok(true);
            }
            let flag = parts[1];
            let id = мяяяяяу_17__(parts[2]).parse::<u64>().unwrap_or(0);
            if id == 0 {
                ctx.http.send_message(message.channel_id, vec![], &serde_json::json!({ "flags": 1<<15, "components": [{ "type": 17, "components": [{ "type": 10, "content": "`[Terminal] Invalid ID`" }] }] })).await.ok();
                return Ok(true);
            }
            if flag == "-m" {
                let _ = message
                    .channel_id
                    .delete_message(ctx, serenity::MessageId::new(id))
                    .await;
            } else if flag == "-c" || flag == "-ch" {
                let _ = serenity::ChannelId::new(id).delete(ctx).await;
            } else {
                ctx.http.send_message(message.channel_id, vec![], &serde_json::json!({ "flags": 1<<15, "components": [{ "type": 17, "components": [{ "type": 10, "content": "`[Terminal] Usage: rm -c|-ch|-m <id>`" }] }] })).await.ok();
                return Ok(true);
            }
            let _ = message.react(ctx, serenity::ReactionType::Unicode("".into())).await;
            return Ok(true);
        }
        "mv" => {
            if parts.len() < 3 {
                ctx.http.send_message(message.channel_id, vec![], &serde_json::json!({ "flags": 1<<15, "components": [{ "type": 17, "components": [{ "type": 10, "content": "`[Terminal] Usage: mv <channel> <category>`" }] }] })).await.ok();
                return Ok(true);
            }
            let ch_id = мяяяяяу_17__(parts[1]).parse::<u64>().unwrap_or(0);
            let cat_id = мяяяяяу_17__(parts[2]).parse::<u64>().unwrap_or(0);
            if ch_id == 0 || cat_id == 0 {
                ctx.http.send_message(message.channel_id, vec![], &serde_json::json!({ "flags": 1<<15, "components": [{ "type": 17, "components": [{ "type": 10, "content": "`[Terminal] Error: Invalid IDs`" }] }] })).await.ok();
                return Ok(true);
            }
            if let Ok(serenity::Channel::Guild(mut ch)) = serenity::ChannelId::new(ch_id).to_channel(ctx).await {
                let _ = ch
                    .edit(ctx, serenity::EditChannel::new().category(serenity::ChannelId::new(cat_id)))
                    .await;
                let _ = message.react(ctx, serenity::ReactionType::Unicode("".into())).await;
            }
            return Ok(true);
        }
        "role" => {
            if parts.len() < 3 {
                ctx.http.send_message(message.channel_id, vec![], &serde_json::json!({ "flags": 1<<15, "components": [{ "type": 17, "components": [{ "type": 10, "content": "`[Terminal] Usage: role <role_id_or_mention> <user_id_or_mention>`" }] }] })).await.ok();
                return Ok(true);
            }
            let role_id = мяяяяяу_17__(parts[1]).parse::<u64>().unwrap_or(0);
            let user_id = мяяяяяу_17__(parts[2]).parse::<u64>().unwrap_or(0);
            if role_id == 0 || user_id == 0 {
                ctx.http.send_message(message.channel_id, vec![], &serde_json::json!({ "flags": 1<<15, "components": [{ "type": 17, "components": [{ "type": 10, "content": "`[Terminal] Error: Invalid IDs`" }] }] })).await.ok();
                return Ok(true);
            }
            if let Ok(member) = guild_id.member(ctx, serenity::UserId::new(user_id)).await {
                let _ = member.add_role(ctx, serenity::RoleId::new(role_id)).await;
                let _ = message.react(ctx, serenity::ReactionType::Unicode("".into())).await;
            }
            return Ok(true);
        }
        "vm" => {
            if parts.len() < 3 {
                ctx.http.send_message(message.channel_id, vec![], &serde_json::json!({ "flags": 1<<15, "components": [{ "type": 17, "components": [{ "type": 10, "content": "`[Terminal] Usage: vm <user_id> <voice_channel_id>`" }] }] })).await.ok();
                return Ok(true);
            }
            let user_id = мяяяяяу_17__(parts[1]).parse::<u64>().unwrap_or(0);
            let voice_id = мяяяяяу_17__(parts[2]).parse::<u64>().unwrap_or(0);
            if user_id == 0 || voice_id == 0 {
                ctx.http.send_message(message.channel_id, vec![], &serde_json::json!({ "flags": 1<<15, "components": [{ "type": 17, "components": [{ "type": 10, "content": "`[Terminal] Error: Invalid IDs`" }] }] })).await.ok();
                return Ok(true);
            }
            let _ = guild_id
                .move_member(ctx, serenity::UserId::new(user_id), serenity::ChannelId::new(voice_id))
                .await;
            let _ = message.react(ctx, serenity::ReactionType::Unicode("".into())).await;
            return Ok(true);
        }
        "flag" => {
            if parts.len() < 4 {
                if let Ok(panel_message) = мяяяяяу_26__(
                    ctx,
                    message.channel_id,
                    мяяяяяу_25__(
                        &lang,
                        &МяуЧерновикФлага {
                            owner_user_id: message.author.id.to_string(),
                            ..Default::default()
                        },
                        None,
                    ),
                ).await {
                    data.terminal_flag_drafts.write().await.insert(
                        panel_message.id.to_string(),
                        МяуЧерновикФлага {
                            owner_user_id: message.author.id.to_string(),
                            ..Default::default()
                        },
                    );
                }
                return Ok(true);
            }
            let action = parts[1];
            let channel_id = мяяяяяу_17__(parts[2]);
            let flag = parts[3].to_lowercase();
            if action == "add" {
                {
                    let mut flags = data.flags.write().await;
                    let list = flags.entry(channel_id.clone()).or_default();
                    if !list.contains(&flag) {
                        list.push(flag.clone());
                    }
                }
                data.мяу_сохрани_flags_29__().await;
                ctx.http.send_message(message.channel_id, vec![], &serde_json::json!({ "flags": 1<<15, "components": [{ "type": 17, "components": [{ "type": 10, "content": format!("`[Terminal]  Added flag '{}' to channel <#{}>`", flag, channel_id) }] }] })).await.ok();
            } else if action == "rm" {
                {
                    let mut flags = data.flags.write().await;
                    let list = flags.entry(channel_id.clone()).or_default();
                    list.retain(|f| f != &flag);
                }
                data.мяу_сохрани_flags_29__().await;
                ctx.http.send_message(message.channel_id, vec![], &serde_json::json!({ "flags": 1<<15, "components": [{ "type": 17, "components": [{ "type": 10, "content": format!("`[Terminal]  Removed flag '{}' from channel <#{}>`", flag, channel_id) }] }] })).await.ok();
            } else {
                ctx.http.send_message(message.channel_id, vec![], &serde_json::json!({ "flags": 1<<15, "components": [{ "type": 17, "components": [{ "type": 10, "content": "`[Terminal] Invalid action.`" }] }] })).await.ok();
            }
            return Ok(true);
        }
        "massrole" => {
            if parts.len() < 2 {
                ctx.http.send_message(message.channel_id, vec![], &serde_json::json!({ "flags": 1<<15, "components": [{ "type": 17, "components": [{ "type": 10, "content": "`[Terminal] Usage: massrole <role_id_or_mention>`" }] }] })).await.ok();
                return Ok(true);
            }
            let role_id = мяяяяяу_17__(parts[1]).parse::<u64>().unwrap_or(0);
            if role_id == 0 {
                ctx.http.send_message(message.channel_id, vec![], &serde_json::json!({ "flags": 1<<15, "components": [{ "type": 17, "components": [{ "type": 10, "content": "`[Terminal] Error: Invalid role`" }] }] })).await.ok();
                return Ok(true);
            }
            ctx.http.send_message(message.channel_id, vec![], &serde_json::json!({ "flags": 1<<15, "components": [{ "type": 17, "components": [{ "type": 10, "content": "`[Terminal] Выдача роли всем участникам...`" }] }] })).await.ok();
            let mut ok = 0;
            let mut fail = 0;
            use poise::futures_util::StreamExt;
            let mut members = guild_id.members_iter(ctx).boxed();
            while let Some(next) = members.next().await {
                if let Ok(member) = next {
                    if !member.user.bot && !member.roles.contains(&serenity::RoleId::new(role_id)) {
                        if member.add_role(ctx, serenity::RoleId::new(role_id)).await.is_ok() {
                            ok += 1;
                        } else {
                            fail += 1;
                        }
                    }
                }
            }
            ctx.http.send_message(message.channel_id, vec![], &serde_json::json!({ "flags": 1<<15, "components": [{ "type": 17, "components": [{ "type": 10, "content": format!("`[Terminal] Выдача завершена. Успешно: {}, Ошибок: {}.`", ok, fail) }] }] })).await.ok();
            return Ok(true);
        }
        "rrole" => {
            if parts.len() < 4 {
                ctx.http.send_message(message.channel_id, vec![], &serde_json::json!({ "flags": 1<<15, "components": [{ "type": 17, "components": [{ "type": 10, "content": "`[Terminal] Usage: rrole @role <emoji> #channel [message]`" }] }] })).await.ok();
                return Ok(true);
            }
            let role_id = мяяяяяу_17__(parts[1]).parse::<u64>().unwrap_or(0);
            let emoji = parts[2].to_string();
            let channel_id = мяяяяяу_17__(parts[3]).parse::<u64>().unwrap_or(0);
            if role_id == 0 || channel_id == 0 {
                ctx.http.send_message(message.channel_id, vec![], &serde_json::json!({ "flags": 1<<15, "components": [{ "type": 17, "components": [{ "type": 10, "content": "`[Terminal] Error: Invalid arguments.`" }] }] })).await.ok();
                return Ok(true);
            }
            let content = if parts.len() > 4 {
                parts[4..].join(" ")
            } else {
                format!("React with {} to get the role with ID {}!", emoji, role_id)
            };
            let posted = serenity::ChannelId::new(channel_id).say(ctx, &content).await?;
            let reaction_type = serenity::ReactionType::Unicode(emoji.clone());
            if posted.react(ctx, reaction_type).await.is_ok() {
                {
                    let mut reaction_roles = data.reaction_roles.write().await;
                    reaction_roles.insert(
                        posted.id.to_string(),
                        serde_json::json!({ "roleId": role_id.to_string(), "emoji": emoji }),
                    );
                }
                data.мяу_сохрани_rroles_30__().await;
                let _ = message.react(ctx, serenity::ReactionType::Unicode("".into())).await;
            }
            return Ok(true);
        }
        "rtr" | "rcr" => {
            if parts.len() < 3 {
                ctx.http.send_message(message.channel_id, vec![], &serde_json::json!({ "flags": 1<<15, "components": [{ "type": 17, "components": [{ "type": 10, "content": "`[Terminal] Usage: rtr <@old_role> <@new_role>`" }] }] })).await.ok();
                return Ok(true);
            }
            let old_role = мяяяяяу_17__(parts[1]).parse::<u64>().unwrap_or(0);
            let new_role = мяяяяяу_17__(parts[2]).parse::<u64>().unwrap_or(0);
            if old_role == 0 || new_role == 0 {
                ctx.http.send_message(message.channel_id, vec![], &serde_json::json!({ "flags": 1<<15, "components": [{ "type": 17, "components": [{ "type": 10, "content": "`[Terminal] Error: Invalid role input.`" }] }] })).await.ok();
                return Ok(true);
            }
            use poise::futures_util::StreamExt;
            let mut ok = 0;
            let mut fail = 0;
            let mut members = guild_id.members_iter(ctx).boxed();
            while let Some(next) = members.next().await {
                if let Ok(member) = next {
                    if member.roles.contains(&serenity::RoleId::new(old_role)) {
                        if member.remove_role(ctx, serenity::RoleId::new(old_role)).await.is_err()
                            || member.add_role(ctx, serenity::RoleId::new(new_role)).await.is_err()
                        {
                            fail += 1;
                        } else {
                            ok += 1;
                        }
                    }
                }
            }
            ctx.http.send_message(message.channel_id, vec![], &serde_json::json!({ "flags": 1<<15, "components": [{ "type": 17, "components": [{ "type": 10, "content": format!("`[Terminal] Complete. Successfully swapped {} members. Errors: {}.`", ok, fail) }] }] })).await.ok();
            return Ok(true);
        }
        "help" => {
            мяяяяяу_26__(ctx, message.channel_id, мяяяяяу_23__(&lang, "help")).await.ok();
            return Ok(true);
        }
        _ => {
            message
                .channel_id
                .say(ctx, мяу!(&lang, "unknown_command"))
                .await?;
            return Ok(true);
        }
    }
}
