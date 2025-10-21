use uuid::Uuid;
use yew::prelude::*;

use crate::player::Player;
use crate::role_reveal::{Role, RoleReveal};
use crate::timer::Timer;
use crate::game_end::GameEnd;

#[derive(Properties, PartialEq, Default)]
pub struct GameProps {
    pub min_imposters: i32,
    pub max_imposters: i32,
    pub game_time: i32,
    pub players: Vec<(Uuid, Player)>,
    pub on_game_finished: Callback<()>,
}

#[derive(Clone, Copy, PartialEq)]
pub enum GameState {
    Revealing,
    Timer,
    End,
}

struct WordToGuess {
    word: AttrValue,
    tipp: AttrValue,
}

impl WordToGuess {
    fn random() -> WordToGuess {
    let words_with_clues = vec![
    ("Tisch", "Möbel"),
    ("Stuhl", "Sitzen"),
    ("Lampe", "Licht"),
    ("Bett", "Schlafen"),
    ("Fenster", "Blick"),
    ("Tür", "Öffnen"),
    ("Auto", "Fahren"),
    ("Handy", "Telefon"),
    ("Uhr", "Zeit"),
    ("Schlüssel", "Tür"),
    ("Kaffeetasse", "Trinken"),
    ("Brot", "Backen"),
    ("Kühlschrank", "Kalt"),
    ("Hemd", "Kleidung"),
    ("Jacke", "Winter"),
    ("Schuhe", "Laufen"),
    ("Hose", "Kleidung"),
    ("Brille", "Sehen"),
    ("Bruder", "Familie"),
    ("Schwester", "Familie"),
    ("Eltern", "Erziehung"),
    ("Kind", "Wachsen"),
    ("Hund", "Tier"),
    ("Katze", "Tier"),
    ("Vogel", "Fliegen"),
    ("Blume", "Pflanze"),
    ("Baum", "Wald"),
    ("Berg", "Hoch"),
    ("See", "Wasser"),
    ("Fluss", "Wasser"),
    ("Strand", "Meer"),
    ("Haus", "Bauen"),
    ("Wohnung", "Raum"),
    ("Nachbar", "Haus"),
    ("Freund", "Gesellschaft"),
    ("Familie", "Verbindung"),
    ("Lachen", "Freude"),
    ("Weinen", "Trauer"),
    ("Sonne", "Himmel"),
    ("Mond", "Nacht"),
    ("Regen", "Wasser"),
    ("Schnee", "Kalt"),
    ("Wind", "Luft"),
    ("Licht", "Tag"),
    ("Dunkelheit", "Nacht"),
    ("Musik", "Klang"),
    ("Film", "Kino"),
    ("Buch", "Lesen"),
    ("Bild", "Kunst"),
    ("Stadt", "Leben"),
    ("Land", "Erde"),
    ("Straße", "Fahrt"),
    ("Platz", "Raum"),
    ("Park", "Grün"),
    ("Bauernhof", "Tiere"),
    ("Küche", "Kochen"),
    ("Bad", "Dusche"),
    ("Wasser", "Trinken"),
    ("Milch", "Getränk"),
    ("Frucht", "Obst"),
    ("Gemüse", "Essen"),
    ("Reis", "Korn"),
    ("Nudel", "Essen"),
    ("Pizza", "Italienisch"),
    ("Salat", "Frisch"),
    ("Bier", "Alkohol"),
    ("Wein", "Alkohol"),
    ("Kaffee", "Heiß"),
    ("Tee", "Heiß"),
    ("Zucker", "Süß"),
    ("Salz", "Würzen"),
    ("Brot", "Backen"),
    ("Kuchen", "Süß"),
    ("Eis", "Kalt"),
    ("Schokolade", "Süß"),
    ("Zahnbürste", "Mundpflege"),
    ("Seife", "Reinigen"),
    ("Shampoo", "Haar"),
    ("Handtuch", "Trocken"),
    ("Toilette", "Bad"),
    ("Dusche", "Wasser"),
    ("Badezimmer", "Waschen"),
    ("Kamm", "Haare"),
    ("Föhn", "Haarpflege"),
    ("Tasche", "Transport"),
    ("Koffer", "Reisen"),
    ("Rucksack", "Schulbedarf"),
    ("Schulheft", "Schreiben"),
    ("Stift", "Schreiben"),
    ("Papier", "Schreiben"),
    ("Bleistift", "Schreiben"),
    ("Notiz", "Schreiben"),
    ("Computer", "Arbeiten"),
    ("Laptop", "Arbeiten"),
    ("Tablet", "Technologie"),
    ("Fernseher", "Sehen"),
    ("Radio", "Hören"),
    ("Kamera", "Fotografie"),
    ("Mikrofon", "Ton"),
    ("Lautsprecher", "Ton"),
    ("Bühne", "Theater"),
    ("Künstler", "Malen"),
    ("Schauspieler", "Film"),
    ("Regisseur", "Film"),
    ("Lied", "Musik"),
    ("Band", "Musik"),
    ("Orchester", "Musik"),
    ("Spiel", "Spaß"),
    ("Spielzeug", "Kinder"),
    ("Puppe", "Spielzeug"),
    ("Auto", "Fahren"),
    ("Zug", "Reisen"),
    ("Flughafen", "Reisen"),
    ("Karte", "Reisen"),
    ("Landkarte", "Welt"),
    ("Reisepass", "Reisen"),
    ("Ticket", "Eintritt"),
    ("Urlaub", "Reisen"),
    ("Ferien", "Schule"),
    ("Reisen", "Abenteuer"),
    ("Welt", "Erde"),
    ("Universum", "Raum"),
    ("Planeten", "Weltraum"),
    ("Sterne", "Himmel"),
    ("Galaxie", "Raum"),
    ("Astronaut", "Raum"),
    ("Rakete", "Weltraum"),
    ("Mond", "Nacht"),
    ("Sonne", "Tag"),
    ("Licht", "Sonne"),
    ("Licht", "Helligkeit"),
    ("Himmel", "Wolken"),
    ("Wolken", "Himmel"),
    ("Regenbogen", "Farbe"),
    ("Blitze", "Sturm"),
    ("Sturm", "Wetter"),
    ("Hurrikan", "Sturm"),
    ("Erdbeben", "Natur"),
    ("Vulkan", "Erde"),
    ("Erde", "Planet"),
    ("Feuer", "Hitze"),
    ("Wasser", "Flüssigkeit"),
    ("Luft", "Atem"),
    ("Gas", "Luft"),
    ("Dampf", "Flüssigkeit"),
    ("Schnee", "Kalt"),
    ("Eis", "Kalt"),
    ("Kälte", "Winter"),
    ("Hitze", "Sommer"),
    ("Sommer", "Jahreszeit"),
    ("Winter", "Kälte"),
    ("Frühling", "Blüten"),
    ("Herbst", "Laub"),
    ("Jahr", "Zeit"),
    ("Woche", "Kalender"),
    ("Tag", "Zeit"),
    ("Monat", "Kalender"),
    ("Uhr", "Zeit"),
    ("Sekunde", "Zeit"),
    ("Minute", "Zeit"),
    ("Jahrhundert", "Zeitalter"),
    ("Ewigkeit", "Zeit"),
    ("Apfel", "Obst"),
    ("Mensch", "Lebewesen"),
    ("Fahrrad", "Verkehr"),
    ("Baum", "Natur"),
    ("Computer", "Technologie"),
    ("Politik", "Gesellschaft"),
    ("Bücher", "Wissen"),
    ("Gitarre", "Instrument"),
    ("Umwelt", "Natur"),
    ("Demokratie", "Staat"),
    ("Gleichheit", "Recht"),
    ("Freiheit", "Recht"),
    ("Arbeiter", "Beruf"),
    ("Solidarität", "Hilfe"),
    ("Kritik", "Bewertung"),
    ("Klima", "Erwärmung"),
    ("Veränderung", "Wandel"),
    ("System", "Struktur"),
    ("Revolution", "Aufstand"),
    ("Wirtschaft", "Markt"),
    ("Feminismus", "Gleichstellung"),
    ("Protest", "Widerstand"),
    ("Energie", "Strom"),
    ("Ressourcen", "Materialien"),
    ("Gegner", "Konflikt"),
    ("Gesetz", "Recht"),
    ("Zukunft", "Morgen"),
    ("Solidarisch", "Unterstützung"),
    ("Autonomie", "Selbstbestimmung"),
    ("Erfindung", "Innovation"),
    ("Verantwortung", "Pflicht"),
    ("Widerstand", "Kampf"),
    ("Gleichberechtigung", "Chancen"),
    ("Bildung", "Schule"),
    ("Geld", "Währung"),
    ("Freundschaft", "Vertrauen"),
    ("Bürgerrecht", "Freiheit"),
    ("Solidarität", "Verbindung"),
    ("Zivilcourage", "Mut"),
    ("Armut", "Mangel"),
    ("Verbraucher", "Konsument"),
    ("Kooperation", "Zusammenarbeit"),
    ("Rassismus", "Diskriminierung"),
    ("Nachhaltigkeit", "Ökologie"),
    ("Integration", "Einbindung"),
    ("Diversity", "Vielfalt"),
    ("Frieden", "Krieg"),
    ("Gerechtigkeit", "Fairness"),
    ("Politische Parteien", "Ideologie"),
    ("Protestbewegung", "Demonstration"),
    ("Arbeitsplatz", "Büro"),
    ("Wissenschaft", "Forschung"),
    ("Zivilgesellschaft", "Gemeinschaft"),
    ("Multilateralismus", "Kooperation"),
    ("Gesundheit", "Krankheit"),
    ("Menschlichkeit", "Mitgefühl"),
    ("Digitalisierung", "Internet"),
    ("Autarkie", "Unabhängigkeit"),
    ("Flüchtling", "Asyl"),
    ("Migration", "Bewegung"),
    ("Diversität", "Unterschiede"),
    ("Gesetzgebung", "Parlament"),
    ("Soziale Gerechtigkeit", "Recht"),
    ("Transparenz", "Offenheit"),
];
        let rand_w_t_g =  words_with_clues[get_random_indices(&words_with_clues, 1)[0]];
        WordToGuess { word: AttrValue::from(rand_w_t_g.0), tipp: AttrValue::from(rand_w_t_g.1) }
    }
}

#[function_component]
pub fn Game(props: &GameProps) -> Html {
    let game_state = use_state(|| GameState::Revealing);
    let imposter_amount = use_state(|| {
    let rand_value = js_sys::Math::random() * (props.max_imposters - props.min_imposters + 1) as f64; // Random float between 0 and (m - n + 1)
        let random_number = rand_value.floor() as usize + (props.min_imposters as usize); // Add `n` to shift the range
        random_number 
    });
    let imposter_indices = use_state(|| get_random_indices(&props.players, *imposter_amount));
    let cycle_reveal_index = use_state(|| 0);
    let to_guess = use_state(|| WordToGuess::random());

    let on_role_reveal_finished = {
        let index = cycle_reveal_index.clone();
        let game_state = game_state.clone();
        let players = props.players.clone();
        move |_| {
            index.set(*index + 1);
            if *index >= players.len() - 1 {
                game_state.set(GameState::Timer);
            }
        }
    };

    let on_timer_finished = {
        let game_state = game_state.clone();
        move |_| {
            game_state.set(GameState::End);
        }
    };

    html! {
        <>
        if *game_state == GameState::Revealing {
            <RoleReveal 
                role={
                    if (*imposter_indices).contains(&*cycle_reveal_index) {
                        Role::Imposter(to_guess.tipp.clone()) // Here goes the tip
                    } else {
                        Role::Guesser(to_guess.word.clone()) // Here goes the word
                    }
                }
                player_name={props.players[*cycle_reveal_index].1.name.clone()}
                {on_role_reveal_finished}
            />
        } else if *game_state == GameState::Timer {
            <Timer 
                time_in_s={props.game_time as u32}
                starting_player_name={AttrValue::from("TODO: Random anfangsspieler")}
                {on_timer_finished}
                
            />
        } else if *game_state == GameState::End {
            <GameEnd
                on_new_game={props.on_game_finished.clone()} 
                imposters={(*imposter_indices).iter().map(|&imposter_index| props.players[imposter_index].1.name.clone()).collect::<Vec<AttrValue>>()}
                word={(*to_guess).word.clone()}
            />
        }
        </>
    }
}

pub fn get_random_indices<T>(vec: &Vec<T>, n: usize) -> Vec<usize> {
    let mut random_indices = Vec::new();
    let len = vec.len();

    // Generate `n` unique random indices
    while random_indices.len() < n {
        let rand_index = js_sys::Math::random() * len as f64;
        let index = rand_index.floor() as usize;

        // Avoid duplicates by checking if index already exists
        if !random_indices.contains(&index) {
            random_indices.push(index);
        }
    }

    random_indices
}