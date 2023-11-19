//! Dependency changes! A way to track the evolution of the user's dependency configuration over time 
//! in a config type agnostic way. They could switch from using a pyproject to a requirements.txt or setup.py
//! and shit should remain the same on how we track the information.
//! This involves: <br>
//! - encoding dependency info to a format and saving it to /changes <br>
//! - decoding it 
//! 
//! the change names are the hashes of the config file for easier change discovery.<br>
//! each change begins with the hash of the previous change but instead of delta encoding <br>
//! each file has the full dependency info of the time because i do not wanna do too many I/O calls
//! or deal with delta encoding for such a small project.

