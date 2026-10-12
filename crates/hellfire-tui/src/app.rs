use hellfire_core::{Deck, DeckStats, Format};


pub struct App {
    pub deck: Deck,
    pub search_results: Vec<String>,
    pub selected_result: usize,
    pub status: String,
    pub status_is_error: bool,
    pub quit: bool,
}

impl App {
    pub fn new() -> Self {
        Self {
            deck: Deck::new("Deck sans nom", Format::Commander),
            search_results: Vec::new(),
            selected_result: 0,
            status: String::from("Prêt à enflammer le meta."),
            status_is_error: false,
            quit: false,
        }
    }

    pub fn draw_stats(&self) -> DeckStats {
        self.deck.stats()
    }

    pub fn set_status(&mut self, msg: String, error: bool) {
        self.status = msg;
        self.status_is_error = error;
    }

    /// « Lever le poing » : valider le deck.
    pub fn raise_fist(&mut self) {
        match self.deck.validate() {
            Ok(stats) => self.set_status(
                format!(
                    "Deck validé : {} cartes, CMC moyen {:.2}. Le poing est levé.",
                    stats.total_cards, stats.avg_cmc
                ),
                false,
            ),
            Err(e) => self.set_status(e.to_string(), true),
        }
    }

    pub fn next_result(&mut self) {
        if !self.search_results.is_empty() {
            self.selected_result = (self.selected_result + 1) % self.search_results.len();
        }
    }

    pub fn prev_result(&mut self) {
        if !self.search_results.is_empty() {
            self.selected_result =
                (self.selected_result + self.search_results.len() - 1) % self.search_results.len();
        }
    }

}
