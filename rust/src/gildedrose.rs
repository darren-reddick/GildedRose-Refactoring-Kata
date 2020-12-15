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

    fn update_quality_brie(item: &mut Item) {
        if item.sell_in > 0 {
            item.quality = std::cmp::min(item.quality + 1, 50);
        };

        if item.sell_in <= 0 {
            item.quality = std::cmp::min(item.quality + 2, 50);
        }

        item.sell_in -= 1;
    }

    fn update_quality_ticket(item: &mut Item) {
        if item.sell_in <= 0 {
            item.quality = 0;
            item.sell_in -= 1;
            return;
        };

        if item.sell_in <= 5 {
            item.quality = std::cmp::min(item.quality + 3, 50);
            item.sell_in -= 1;
            return;
        }

        if item.sell_in <= 10 {
            item.quality = std::cmp::min(item.quality + 2, 50);
            item.sell_in -= 1;
            return;
        }

        item.quality += 1;
        item.sell_in -= 1;
    }

    fn update_quality_standard(item: &mut Item) {
        if item.sell_in <= 0 {
            item.quality = std::cmp::max(item.quality - 2, 0);
            item.sell_in -= 1;
            return;
        }

        item.quality = std::cmp::max(item.quality - 1, 0);
        item.sell_in -= 1;
    }

    fn update_quality_conjured(item: &mut Item) {
        if item.sell_in <= 0 {
            item.quality = std::cmp::max(item.quality - 4, 0);
            item.sell_in -= 1;
            return;
        }

        item.quality = std::cmp::max(item.quality - 2, 0);
        item.sell_in -= 1;
    }

    pub fn update_quality(&mut self) {
        println!("updating quality");
        for item in &mut self.items {
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
    pub fn test_all() {
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

            println!("Testing {}", test.0);
            println!(
                "Day {}: sellin: {} quality: {}",
                0, rose.items[0].sell_in, rose.items[0].quality
            );

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
