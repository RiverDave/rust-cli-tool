//===----------------------------------------------------------------------===//
//
// Copyright (c) 2025 David Rivera
//
// This source code is licensed under the MIT license found in the
// LICENSE file in the root directory of this source tree.
//
// SPDX-License-Identifier: MIT
//
//===----------------------------------------------------------------------===//
//
// Unit tests for output module
//===----------------------------------------------------------------------===//
//

#[cfg(test)]
mod tests {
    use cli_rust::{OutputDestination, OutputFormat};

    #[test]
    fn test_output_format_extensions() {
        assert_eq!(OutputFormat::Plain.to_extension(), "txt");
        assert_eq!(OutputFormat::Json.to_extension(), "json");
        assert_eq!(OutputFormat::Markdown.to_extension(), "md");
    }

    #[test]
    fn test_output_format_enum_variants() {
        // Test that all enum variants work correctly
        let formats = [
            OutputFormat::Plain,
            OutputFormat::Json,
            OutputFormat::Markdown,
        ];

        for format in formats.iter() {
            // Should not panic
            let _extension = format.to_extension();
            let _debug_str = format!("{:?}", format);
        }
    }

    #[test]
    fn test_output_destination_variants() {
        let stdout_dest = OutputDestination::Stdout;
        let file_dest = OutputDestination::File("test.txt".to_string());

        // Test that we can match on variants
        match stdout_dest {
            OutputDestination::Stdout => { /* Expected */ }
            OutputDestination::File(_) => panic!("Expected Stdout"),
        }

        match file_dest {
            OutputDestination::File(path) => assert_eq!(path, "test.txt"),
            OutputDestination::Stdout => panic!("Expected File"),
        }
    }

    #[test]
    fn test_output_destination_clone() {
        let dest1 = OutputDestination::File("output.md".to_string());
        let dest2 = dest1.clone();

        match (dest1, dest2) {
            (OutputDestination::File(path1), OutputDestination::File(path2)) => {
                assert_eq!(path1, path2);
            }
            _ => panic!("Clone should preserve variant and data"),
        }
    }

    #[test]
    fn test_output_format_clone() {
        let format1 = OutputFormat::Markdown;
        let format2 = format1.clone();

        assert_eq!(format1.to_extension(), format2.to_extension());
    }

    #[test]
    fn test_output_enums_debug() {
        let format = OutputFormat::Json;
        let destination = OutputDestination::File("test.json".to_string());

        let format_debug = format!("{:?}", format);
        let dest_debug = format!("{:?}", destination);

        assert!(format_debug.contains("Json"));
        assert!(dest_debug.contains("test.json"));
    }
}
