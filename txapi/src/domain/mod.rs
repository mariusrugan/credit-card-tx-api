pub mod prelude {
    pub use super::{
        heartbeat::Heartbeat,
        transactions::{Transaction, TransactionCategory},
    };
}

pub mod transactions {
    use rand::Rng;
    use serde::{Deserialize, Serialize};

    /// Category of merchant for a transaction.
    ///
    #[derive(Deserialize, Serialize, Debug, Clone, Copy, PartialEq, Eq)]
    pub enum TransactionCategory {
        #[serde(rename = "grocery")]
        Grocery,
        #[serde(rename = "gas_station")]
        GasStation,
        #[serde(rename = "restaurant")]
        Restaurant,
        #[serde(rename = "online_retail")]
        OnlineRetail,
        #[serde(rename = "entertainment")]
        Entertainment,
        #[serde(rename = "travel")]
        Travel,
        #[serde(rename = "healthcare")]
        Healthcare,
        #[serde(rename = "utilities")]
        Utilities,
    }

    impl TransactionCategory {
        /// Returns all available categories.
        pub fn all() -> Vec<Self> {
            vec![
                Self::Grocery,
                Self::GasStation,
                Self::Restaurant,
                Self::OnlineRetail,
                Self::Entertainment,
                Self::Travel,
                Self::Healthcare,
                Self::Utilities,
            ]
        }

        /// Returns a random category with weighted distribution.
        pub fn random() -> Self {
            let mut rng = rand::rng();
            let val: f64 = rng.random();

            // Weighted distribution based on typical transaction patterns
            match val {
                v if v < 0.25 => Self::Grocery,
                v if v < 0.40 => Self::Restaurant,
                v if v < 0.55 => Self::GasStation,
                v if v < 0.70 => Self::OnlineRetail,
                v if v < 0.80 => Self::Entertainment,
                v if v < 0.90 => Self::Utilities,
                v if v < 0.95 => Self::Travel,
                _ => Self::Healthcare,
            }
        }

        /// Returns typical amount range for this category (min, max) in cents.
        pub fn typical_amount_range(&self) -> (u64, u64) {
            match self {
                Self::Grocery => (500, 15000),       // $5 - $150
                Self::GasStation => (2000, 8000),    // $20 - $80
                Self::Restaurant => (1000, 12000),   // $10 - $120
                Self::OnlineRetail => (1500, 25000), // $15 - $250
                Self::Entertainment => (1000, 20000), // $10 - $200
                Self::Travel => (5000, 100000),      // $50 - $1000
                Self::Healthcare => (3000, 50000),   // $30 - $500
                Self::Utilities => (5000, 30000),    // $50 - $300
            }
        }
    }

    /// Geographic location data for a transaction.
    ///
    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct Location {
        /// City name
        pub city: String,
        /// ISO 3166-1 alpha-2 country code
        pub country_iso: String,
        /// Latitude coordinate (-90 to 90)
        pub latitude: f64,
        /// Longitude coordinate (-180 to 180)
        pub longitude: f64,
    }

    impl Location {
        /// Returns a random location from a predefined set of cities.
        pub fn random() -> Self {
            let locations = vec![
                ("San Francisco", "US", 37.774929, -122.419418),
                ("New York", "US", 40.712776, -74.005974),
                ("Los Angeles", "US", 34.052235, -118.243683),
                ("Chicago", "US", 41.878113, -87.629799),
                ("Miami", "US", 25.761681, -80.191788),
                ("London", "GB", 51.507351, -0.127758),
                ("Paris", "FR", 48.856613, 2.352222),
                ("Tokyo", "JP", 35.689487, 139.691711),
                ("Sydney", "AU", -33.868820, 151.209290),
                ("Toronto", "CA", 43.651070, -79.347015),
            ];

            let mut rng = rand::rng();
            let (city, country, lat, lon) = locations[rng.random_range(0..locations.len())];

            Self {
                city: city.to_string(),
                country_iso: country.to_string(),
                latitude: lat,
                longitude: lon,
            }
        }
    }

    /// Domain model for a Credit Card Transaction.
    ///
    /// Represents a mock credit card transaction with realistic fields
    /// useful for fraud detection and data analysis practice.
    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct Transaction {
        /// Unique transaction identifier (32 hex characters)
        pub id: String,

        /// Transaction timestamp in RFC3339 format
        pub timestamp: String,

        /// Credit card number (mock data only)
        pub cc_number: String,

        /// Merchant category
        pub category: TransactionCategory,

        /// Transaction amount in USD cents
        /// Example: 4599 represents $45.99
        pub amount_usd_cents: u64,

        /// Geographic location of the transaction
        #[serde(flatten)]
        pub location: Location,

        /// Whether the transaction was made online
        pub is_online: bool,
    }

    impl Transaction {
        /// Creates a realistic mock transaction with randomized values.
        ///
        /// Generates transactions with:
        /// - Valid Luhn-checksum credit card numbers
        /// - Varied locations across multiple cities
        /// - Category-appropriate amounts
        /// - Realistic online/offline distribution
        ///
        pub fn simple_mock() -> Self {
            let category = TransactionCategory::random();
            let location = Location::random();
            let (min_amount, max_amount) = category.typical_amount_range();
            let mut rng = rand::rng();

            let amount_usd_cents = rng.random_range(min_amount..=max_amount);
            let is_online = rng.random_bool(0.3); // 30% of transactions are online

            Self {
                id: uuid::Uuid::new_v4().to_string().replace("-", ""),
                timestamp: chrono::Utc::now().to_rfc3339(),
                cc_number: Self::generate_valid_cc_number(),
                category,
                amount_usd_cents,
                location,
                is_online,
            }
        }

        /// Generates a valid credit card number with Luhn checksum.
        ///
        /// Uses the Visa prefix (4) and generates 15 random digits
        /// plus a valid checksum digit.
        fn generate_valid_cc_number() -> String {
            let mut rng = rand::rng();
            let mut digits: Vec<u8> = vec![4]; // Visa prefix

            // Generate 14 random digits
            for _ in 0..14 {
                digits.push(rng.random_range(0..10));
            }

            // Calculate Luhn checksum
            let checksum = Self::calculate_luhn_checksum(&digits);
            digits.push(checksum);

            digits.iter().map(|d| d.to_string()).collect()
        }

        /// Calculates the Luhn checksum digit for a sequence of digits.
        fn calculate_luhn_checksum(digits: &[u8]) -> u8 {
            let sum: u32 = digits
                .iter()
                .rev()
                .enumerate()
                .map(|(idx, &digit)| {
                    let mut d = digit as u32;
                    if idx % 2 == 0 {
                        // Double every other digit from right
                        d *= 2;
                        if d > 9 {
                            d -= 9;
                        }
                    }
                    d
                })
                .sum();

            ((10 - (sum % 10)) % 10) as u8
        }
    }
}

pub mod heartbeat {
    use serde::{Deserialize, Serialize};

    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct Heartbeat {
        pub status: String,
    }
}
