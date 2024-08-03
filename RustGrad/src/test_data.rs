use super::*; // Import `generate_data` and `RNG` (if in the same module)
use crate::utils::RNG;


#[cfg(test)]
mod tests {
    use crate::utils::RNG; // Import `RNG` if in a separate module

    #[test]
    fn test_generate_data_size() {
        let mut rng = RNG::new(42); // Seed the RNG for consistent results
        let (train_data, val_data, test_data) = generate_data(&mut rng, 100);

        let total_data = train_data.len() + val_data.len() + test_data.len();
        assert_eq!(total_data, 100, "Total data points should be 100");
    }

    #[test]
    fn test_generate_data_labels() {
        let mut rng = RNG::new(42); // Seed the RNG for consistent results
        let (train_data, _, _) = generate_data(&mut rng, 10);

        // Check for presence of all labels (0, 1, and 2)
        let mut label_counts = [0; 3]; // Array to store label counts
        for point in &train_data {
            label_counts[point.label] += 1;
        }

        assert!(label_counts.iter().all(|&count| count > 0), "All labels should be present");
    }

    // Add more tests for specific aspects of data generation
}