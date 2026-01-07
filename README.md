# About

This is a **Linux-only utility** that transforms files so they cannot be used without a key.

- Works on **raw bytes**, not on file formats  
- Without the correct key, the file becomes **unusable**  
- With the correct key, the original file is restored **exactly**

---

# Platform

- **OS:** GNU/Linux  
- **Architecture:** x86_64  
- **Format:** ELF  

Other systems are **not supported**.

---

# Important Notice

⚠️ **This program is NOT cryptography.**

- No AES  
- No RSA  
- No security guarantees  

**Do not use it to protect sensitive data.**

---

# How It Works (Short)

1. Takes a file  
2. Applies a key-based transformation  
3. Breaks the file structure  
4. Reverses the process only with the correct key  

That’s it.

---

# Files

The program works with **any file type**, because it does not care what the file is:

- Binaries  
- Archives  
- Images  
- Text files  

> **If a file stops opening without the key, that is expected behavior.**

---

# License

**GPL-3.0**

You may:
- Use  
- Modify  
- Redistribute  

Derivatives must remain **open source**.

---

# Notes

This is an **experimental project** for Linux.

No promises.  
No guarantees.
