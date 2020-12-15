use std::fmt::{self, Display};

#[derive(Debug)]
pub struct Item {
    pub name: String,
    pub sell_in: i32,
    pub quality: i32,
}

impl Item {
    pub fn new(name: impl Into<String>, sell_in: i32, quality: i32) -> Item {
        Item {
            name: name.into(),
            sell_in,
            quality,
        }
    }
}

impl Display for Item {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}, {}, {}", self.name, self.sell_in, self.quality)
    }
}

pub struct GildedRose {
    pub items: Vec<Item>,
}

impl GildedRose {
    pub fn new(items: Vec<Item>) -> GildedRose {
        GildedRose { items }
    }

    // method to update quality of brie items
    fn update_quality_brie(item: &mut Item) {
        // increase quality by 1 as sellin date approaches
        if item.sell_in > 0 {
            item.quality = std::cmp::min(item.quality + 1, 50); // using std min function to ensure max quality is 50
        };

        // increase quality by 2 after sellin date has arrived
        if item.sell_in <= 0 {
            item.quality = std::cmp::min(item.quality + 2, 50); // using std min function to ensure max quality is 50
        }

        item.sell_in -= 1;
    }

    // method to update quality of ticket items
    fn update_quality_ticket(item: &mut Item) {
        // set to quality to 0 if sellin date has arrived
        if item.sell_in <= 0 {
            item.quality = 0;
            item.sell_in -= 1;
            return;
        };

        // increase value by 3 with less than 3 days to sell
        if item.sell_in <= 5 {
            item.quality = std::cmp::min(item.quality + 3, 50);
            item.sell_in -= 1;
            return;
        }

        // increase value by 2 with less than 10 days to sell
        if item.sell_in <= 10 {
            item.quality = std::cmp::min(item.quality + 2, 50);
            item.sell_in -= 1;
            return;
        }

        // the default: increase quality by 1
        item.quality += 1;
        item.sell_in -= 1;
    }

    // method to update quality of standard items
    fn update_quality_standard(item: &mut Item) {
        // degrade by 2 when sellin date has arrived
        if item.sell_in <= 0 {
            item.quality = std::cmp::max(item.quality - 2, 0); // using std max function to ensure min quality is 0
            item.sell_in -= 1;
            return;
        }

        // degrade by 1 when past sellin date
        item.quality = std::cmp::max(item.quality - 1, 0); // using std max function to ensure min quality is 0
        item.sell_in -= 1;
    }

    // method to update quality of conjured items
    fn update_quality_conjured(item: &mut Item) {
        // degrade by 4 when sellin date has arrived
        if item.sell_in <= 0 {
            item.quality = std::cmp::max(item.quality - 4, 0); // using std max function to ensure min quality is 0
            item.sell_in -= 1;
            return;
        }
        // degrade by 2 before sellin date
        item.quality = std::cmp::max(item.quality - 2, 0); // using std max function to ensure min quality is 0
        item.sell_in -= 1;
    }

    // method to update quality of items available at the gilded rose
    pub fn update_quality(&mut self) {
        println!("updating quality");
        for item in &mut self.items {
            // each arm selects item for update depedent on name and uses handler for that type
            // (really wanted to use a match here but had trouble dereferencing from &item)
            if item.name == "Sulfuras, Hand of Ragnaros" {
                return;
            } else if item.name == "Aged Brie" {
                GildedRose::update_quality_brie(item);
                return;
            } else if item.name == "Backstage passes to a TAFKAL80ETC concert" {
                GildedRose::update_quality_ticket(item);
                return;
            } else if item.name == "Conjured Mana Cake" {
                GildedRose::update_quality_conjured(item);
                return;
            } else {
                GildedRose::update_quality_standard(item);
                return;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{GildedRose, Item};

    #[test]
    pub fn test_items() {
        let tests = vec![
            // tests are of tuple type (name, sellin, quality, expected sellin, expected quality, no. days to update)
            ("Sulfuras, Hand of Ragnaros", 5, 80, 5, 80, 5),
            ("Sulfuras, Hand of Ragnaros", 3, 80, 3, 80, 4),
            ("Aged Brie", 2, 0, -2, 6, 4),
            ("Aged Brie", 5, 50, 4, 50, 1),
            ("Aged Brie", 9, 5, 8, 6, 1),
            ("Aged Brie", 0, 0, -1, 2, 1),
            (
                "Backstage passes to a TAFKAL80ETC concert",
                15,
                49,
                14,
                50,
                1,
            ),
            ("Backstage passes to a TAFKAL80ETC concert", 0, 49, -1, 0, 1),
            ("Backstage passes to a TAFKAL80ETC concert", 4, 4, 3, 7, 1),
            ("Backstage passes to a TAFKAL80ETC concert", 9, 4, 8, 6, 1),
            (
                "Backstage passes to a TAFKAL80ETC concert",
                10,
                49,
                1,
                50,
                9,
            ),
            ("Conjured Mana Cake", 3, 6, 2, 4, 1),
            ("Conjured Mana Cake", 0, 8, -1, 4, 1),
            ("Conjured Mana Cake", 3, 0, 2, 0, 1),
            ("Acme Dynamite", 3, 6, 2, 5, 1),
            ("Acme Dynamite", 0, 6, -3, 0, 3),
            ("Acme Dynamite", 0, 0, -1, 0, 1),
        ];

        for test in tests.iter() {
            let items = vec![Item::new(test.0, test.1, test.2)];

            let mut rose = GildedRose::new(items);

            for i in 0..test.5 {
                rose.update_quality();
                println!(
                    "Day {}: sellin: {} quality: {}",
                    i + 1,
                    rose.items[0].sell_in,
                    rose.items[0].quality
                );
            }

            assert_eq!(
                (test.3, test.4),
                (rose.items[0].sell_in, rose.items[0].quality)
            );
        }
    }
}
