#!/usr/bin/env python3
import unittest
import subprocess
import os
import tempfile
import re

class WaterBinaryTest(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        # Build the binary if not already built
        subprocess.run(["cargo", "build"], check=True)
        cls.binary_path = "./target/debug/water"

    def run_binary(self, input_data=None):
        """Run the binary with optional input data and return stdout and stderr"""
        if input_data:
            # Create a temporary file with the input
            with tempfile.NamedTemporaryFile(mode='w', delete=False) as temp:
                temp.write(input_data)
                temp_path = temp.name
            
            try:
                result = subprocess.run(
                    [self.binary_path, temp_path], 
                    capture_output=True, 
                    text=True
                )
                os.unlink(temp_path)
                return result.stdout, result.stderr, result.returncode
            except Exception as e:
                os.unlink(temp_path)
                raise e
        else:
            # Run without input
            result = subprocess.run(
                [self.binary_path], 
                capture_output=True, 
                text=True
            )
            return result.stdout, result.stderr, result.returncode

    def test_basic_execution(self):
        """Test that the binary runs without crashing"""
        stdout, stderr, returncode = self.run_binary()
        self.assertEqual(returncode, 0, f"Binary failed with return code {returncode}")

    def test_i32_const_parsing(self):
        """Test that i32.const is parsed correctly"""
        stdout, stderr, _ = self.run_binary()
        # Check if the output contains the expected debug output for i32.const
        self.assertIn("i32.const", stderr)
        self.assertIn("5", stderr)

    def test_local_set_parsing(self):
        """Test that local.set is parsed correctly"""
        stdout, stderr, _ = self.run_binary()
        # Check if the output contains the expected debug output for local.set
        self.assertIn("local.set", stderr)
        self.assertIn("$idx", stderr)

    def test_nested_instruction_parsing(self):
        """Test that nested instructions are parsed correctly"""
        stdout, stderr, _ = self.run_binary()
        # Check for the nested instruction "(local.set $idx (i32.const 5))"
        self.assertIn("local.set", stderr)
        self.assertIn("i32.const", stderr)

    def test_instruction_arguments_parsing(self):
        """Test that instructions with arguments are parsed correctly"""
        stdout, stderr, _ = self.run_binary()
        # Check for the argument list in an instruction
        combined_output = stdout + stderr
        self.assertIn("arguments", combined_output)
        self.assertIn("VariableOperation", combined_output)

if __name__ == "__main__":
    unittest.main()