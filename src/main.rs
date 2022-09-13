use lodestone::model::{datacenter::Datacenter, gc::GrandCompany, language::Language};
use lodestone::search::SearchBuilder;
use rusqlite::{named_params, Connection};

fn main() {
    let db: Connection = Connection::open("lodestone.db").expect("failed to open");

    println!("initializing database");
    db.execute_batch(include_str!("init.sql"))
        .expect("failed to run init script");

    println!("searching for character");
    let profiles = SearchBuilder::new()
        .character("Yov Ziv")
        .datacenter(Datacenter::Primal)
        .lang(Language::English)
        .grand_company(GrandCompany::Maelstrom)
        .send()
        .expect("failed to send");

    let myself = profiles.first().expect("failed to get first profile");

    println!("fetched information for {}", myself.name);
    db.execute(
        "INSERT OR IGNORE INTO profile_snapshots (
            user_id, free_company, name, nameday, guardian, city_state, server,
            race, clan, gender, hp, mp
        ) VALUES (
            :user_id, :free_company, :name, :nameday, :guardian, :city_state,
            :server, :race, :clan, :gender, :hp, :mp
        )",
        named_params! {
            ":user_id": myself.user_id,
            ":free_company": myself.free_company,
            ":name": myself.name,
            ":nameday": myself.nameday,
            ":guardian": myself.guardian,
            ":city_state": myself.city_state,
            ":server": format!("{:?}", myself.server),
            ":clan": format!("{:?}", myself.clan),
            ":race": format!("{:?}", myself.race),
            ":gender": format!("{:?}", myself.gender),
            ":hp": myself.hp,
            ":mp": myself.mp
        },
    )
    .expect("failed to insert");

    let snapshot_id = db.last_insert_rowid();
    println!("created snapshot ID: {}", snapshot_id);

    for (class, info) in &myself.classes.0 {
        if let Some(xp) = info {
            db.execute(
                "INSERT INTO experience_snapshots (
                    snapshot_id, class_name, xp_level, current_xp, max_xp
                ) VALUES (
                    :snapshot_id, :class_name, :xp_level, :current_xp, :max_xp
                )",
                named_params! {
                    ":snapshot_id": snapshot_id,
                    ":class_name": format!("{:?}", class),
                    ":xp_level": xp.level,
                    ":current_xp": xp.current_xp,
                    ":max_xp": xp.max_xp,
                },
            )
            .expect("failed to insert experience");

            println!("stored experience: {:?}", class);
        }
    }
}
