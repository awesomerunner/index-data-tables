# Index Data Table (IDT) Specification  

*Version: 1.0*  
*File extension: `.idt`*  

---

## 1. Definition of Terms

| Term | Definition |
| --- | --- |
| **Magic** | The byte sequence identifying an IDT file. Always `0x49 44 54` for valid files. |
| **Version** | The version of the IDT Specification |
| **Size** | The number of bytes in a field |
| **Common Field Size** | The size of all **Length**, **Index**, and **Code** fields, in bytes. The value of the **Common Field Size** is `n` |
| **Validation Block Size** | The size, in bytes of a **Validation Block**. The value of the **Validation Block Size** is `k`. If `k=0`, all bytes following the header are raw file data without validation checks. |
| **Validation Block** | A parity square of **Data Bytes**, plus the associated **Parity Bytes** used to validate and correct the data contained in the block. |
| **Table** | A set of **Entries** with unique indices |
| **Table Entry** | A sequence of bytes containing a **Length** field, followed by an **Index** field, followed by **Data** field. The sizes of the **Data** field is given by the value in the entry's **Length** field. |
| **Instruction Code** | A special sequence that gives instructions to the parsing software. Instruction codes can occur anywhere a **Table Entry** is expected, and are differentiated from **Table Entries** by having a value of `0` where the **Length** field is expected. The specific instruction is then identified by a **Code** field |
| **General Metadata** | Metadata common to all **Tables** in a file |
| **Table Header** | Header for a **Table** in the file, including the **Table ID** and **Data Type ID** |
| **Table ID** | A unique, 32 ID number for a **Table** |
| **Data Type ID** | A 32-bit Murmur3 Hash of a short string uniquely identifying a translation between the contents of **Data** fields in a **Table** and an application data type |
| **Index** | A sequence of `n` bytes encoding the index of a **Data** field. Indexes within a table are unique. |
| **Code** | A field indicating which **Instruction Code** is being called. The **Code** field is **ALWAYS** `2` bytes, big-endian |
| **Data Bytes** | Bytes after the header that are used for the actual data in the file, ie. not **Parity Bytes** |


---

## 2. General Principles

- All multi‑byte numeric values are unsigned and stored **big‑endian** unless otherwise specified.

## 3. File Header (8 bytes)

| Position (From Start of File) | Size | Field | Description |
|--------|------|-------|-------------|
| 0 – 2  | 3 B   | **Magic** | ASCII `"IDT" (0x49 44 54)` – identifies the file as an Index Data Table. |
| 3 – 4  | 2 B   | **Version** | IDT version (e.g., `0x0001`). Version values greater or equal to `0x8000` are reserved for experimental versions, version values `0x0001-0x7FFF` will be used for stable versions |
| 5      | 1 B   | **Common Field Size (n)** | Number of bytes used for each **Index** and **Length** field. Maximum value is `8` |
| 6      | 1 B   | **Validation Block Size (k)** | Side length of the 2‑D parity square; `0` = no validation. Maximum value is `64` to ensure reasonably sized blocks |
| 7      | 1 B   | **Header Parity** | XOR of header bytes 0 through 6 (HeaderParity = `B0 ⊕ … ⊕ B6`). |

#### Header Errors

- If an unexpected value is found in the header, abort with one of the following:
  - `IDT FORMAT ERROR - INVALID FORMAT IDENTIFIER` for invalid **Magic**
  - `IDT FORMAT ERROR - UNSUPPORTED FORMAT VERSION` for an unknown or unsupported version
  - `IDT FORMAT ERROR - UNSUPPORTED INDEX/LENGTH FIELD SIZE` for an unsupported **Common Field Size**
  - `IDT FORMAT ERROR - UNSUPPORTED VALIDATION BLOCK SIZE` for an unsupported **Validation Block Size**
  - `IDT FORMAT ERROR - HEADER VALIDATION FAILURE` if the header parity check fails (The parity check fails if the XOR of all header bytes combined does not equal `0x00`)
- If an EOF occurs during the header, abort parsing with either:
  - `IDT FORMAT ERROR - INCOMPLETE HEADER`

---

## 4. Validation Scheme (when `k > 0`)  

If the **Validation Block Size (k)** is not `0`, all data following the header, including any file or table metadata, is stored in parity squares. Each parity square is accompanied by a series of parity bytes for validation and single-error correction. The combination of a parity square and the associated validation bytes is a **Validation Block**. If k = `0`, **Validation Blocks** are not used and the header is followed by the raw contents of the file.

### 4.1 Validation Block size and Layout 

  `B = k^{2} + (2k) + 1` bytes

- Layout (written sequentially):  

  1. **Data Bytes** – `k × k` bytes (row‑major order). 
  2. **Row Parity Bytes** – `k` bytes (XOR of each row).  
  3. **Column Parity Bytes** – `k` bytes (XOR of each column).  
  4. **Overall Parity Byte** – 1 byte (XOR of all data region bytes, which should be equal to the XOR of all row parity bytes, and the XOR of all column parity bytes).  

### 4.2 Padding and Alignment

- If the raw length of the contents of the file, excluding the header, is not a multiple of `k²`, append the minimum number of `0xff` such that all **Validation Blocks** can be completely filled. The padding step should be completed before the final **Validation Block** is formed.
- The choice of `0xff` is to avoid the parser incorrectly interpreting padding bytes as a **Stop-File Signal**, which may occur if length specifiers are incorrect. Using `0xff` for padding bytes guarantees an error if the data is invalid.
- After validation, if a block contains an end-of-file signal, the remaining bytes after the signal should be checked to ensure they contain only `0xff` bytes. Any non-padding bytes after this signal should be treated as a fatal error.
- Data fields in the file are not guaranteed to be aligned with, or fully contained within, **Validation Blocks**. Parsing of the file should be agnostic with respect to the positioning of **Parity Bytes**. In particular, **Parity Bytes** should not be included when interpreting **Index** and **Length** fields, nor should they be included in the contents of a **Data** field passed to the application. The length of a **Data** field does not include any **Parity Bytes** occuring between the **Data Bytes** making up that field.
  - Parsers are recommended to maintain a virtual stream that de‑interleaves **Parity Bytes**, allowing the reciever of this virtual stream to treat all bytes recieved as **Data Bytes**.

### 4.3 Bit Position Independence

Each bit plane in a **Validation Block** is a Hamming codeword allowing for single-bit error correction and multi-bit error detection. Only single‑bit errors per bit‑plane are guaranteed to be correctable; multiple errors affecting the same bit‑plane may be detectable but not correctable.

#### Validation Errors

- If an EOF occurs in any location other than the end of a **Validation Block**, abort parsing with `IDT VALIDATION ERROR - INCOMPLETE VALIDATION BLOCK

---  

## 5. General Metadata  

The first `n` **Data Bytes** following the header are a **Length** field constitute the **General Metadata Length (m)** field, an unsigned big‑endian integer. The **General Metadata** field follows immediately after those `n` bytes. The **General Metadata Length (m)** field is not part of the **General Metadata** field. The length of the **General Metadata** is counted from that byte.

### 5.1 General Metadata Layout  

| Offset (relative to first byte after Header) | Size (bytes) | Field | Required? | Notes |
|-----------------------------------------------|--------------|-------|-----------|-------|
| 0                                               | `n`       | **General Metadata Length (m)** | Yes | Unsigned big‑endian integer giving the total size of the **General Metadata** field that follows. |
| n                                               | `8`            | **File‑Creation Timestamp** | Yes | Unix epoch seconds, big‑endian – the time this IDT file was written. |
| n + 8                                           | `8`            | **Original‑File Timestamp** | Yes* | Unix epoch seconds, big‑endian. If the **Copied** flag (bit 0 of Flags) is `0`, this value must equal the File‑Creation Timestamp; if the flag is `1`, it records the creation time of the original source of the data. |
| n + 16                                          | `1`            | **Flags** | Yes | Bit‑field (see table below). |
| n + 17                                          | `7`            | **Reserved** | Yes | Must be zero; reserved for future extensions. |
| n + 24                                          | `m-24`         | **Additional** | No | Application-Defined additional general metadata. If used, should include a check to ensure the file was intended to be read by the application.

#### Flags (1 byte)

| Bit | Name   | Meaning |
|-----|--------|---------|
| 0   | **Copied** | `1` if this IDT file was generated by copying (updating) an older IDT file; `0` otherwise. |
| 1‑7 | —      | Reserved (must be zero). |

The fixed size of the required general metadata is `24` bytes. Additional fields may be appended after the Reserved bytes; in that case **General Metadata Length (m)** must be updated to reflect the new total size.

## 5.2 Metadata Errors  

| Condition | Error Code | Message |
|-----------|------------|---------|
| **Length field (`m`) exceeds remaining file size** | `IDT METADATA ERROR` | `GENERAL METADATA LENGTH EXCEEDS FILE BOUNDARY` |
| **File‑Creation Timestamp missing (EOF reached while reading the 8‑byte timestamp)** | `IDT METADATA ERROR` | `INCOMPLETE FILE‑CREATION TIMESTAMP` |
| **Original‑File Timestamp missing (EOF reached while reading the 8‑byte timestamp)** | `IDT METADATA ERROR` | `INCOMPLETE ORIGINAL‑FILE TIMESTAMP` |
| **Flags byte missing (EOF reached while reading the 1‑byte Flags field)** | `IDT METADATA ERROR` | `INCOMPLETE FLAGS FIELD` |
| **Reserved bytes (7 B) not all zero** | `IDT METADATA ERROR` | `NON‑ZERO RESERVED FIELD` |
| **General Metadata Length (`m`) is less than the mandatory 24 bytes** | `IDT METADATA ERROR` | `METADATA LENGTH TOO SMALL` |
| **Any required metadata field (timestamp, flags, reserved) is absent because EOF occurs earlier** | `IDT METADATA ERROR` | `TRUNCATED METADATA SECTION` |
| **Flag bits 1‑7 are set (must be zero)** | `IDT METADATA ERROR` | `RESERVED FLAG BITS SET` |

---

## 6. Tables  

After the end of the **General Metadata** Field, a **Table Header** is required. **Table Headers** are always exactly `8` **Data Bytes** long, and consist of the **Table ID** followed by the **Data Type ID**. The application should check that both components of the table header are valid and match what the application expects.

After the **Table Header**, the following **Data Bytes** encode **Table Entries** or **Instruction Codes**. If the **Data Bytes** in the file are all read before a **Stop-File** code, abort parsing with `IDT PARSING ERROR - UNEXPECTED END-OF-FILE`

### 6.1 Entry structure  

| Component | Size (bytes) | Description |
|-----------|--------------|-------------|
| **Length** | **n** (as defined in Header Byte 5) | Unsigned big‑endian integer. An length of 0 is not allowed for a **Table Entry** and if valid **Code** is not found in the next `2` bytes, abort parsing with `IDT PARSING ERROR - INVALID INSTRUCTION CODE/ZERO LENGTH FIELD`. The value of the **Length** field is the size of the **Data** field, the bytes in the **Length** and **Index** Fields are not counted in the value of the **Length** field. |
| **Index**| **n** (same *n* as above) | Unsigned big‑endian integer uniquely identifying a **Data** field within a **Table** If an **Index** appears more than once within a **Table**, abort parsing with `IDT PARSING ERROR - DUPLICATE INDEX` |
| **Data**  | Value of **Length** field | Raw payload bytes. At least one byte is always present. |

### 6.2 Instruction Codes  

- An **Instruction Code** entry is identified by value of `0` where a **Length** is expected. The `2` byte **Code** field following the identifier determines the code to be resolved. 
- The only instruction defined today is the **Stop‑Parsing** command:  

| **Code** Field Value | Name | Description |
| -------------------- | ---- | ----------- |
| `0xFF FF` | UNUSED | Never used, forces a fatal error |
| `0x00 00` | STOP FILE | Marks end of IDT data, all following **Data Bytes** should be `0xFF` padding |
| `0x00 01` | NEW TABLE | Marks end of table, following bits should be interpreted as the **Table Header** for a new **Table** |

All other **Code** values are reserved for future extensions. If an unknown Code is read, abort with `IDT PARSING ERROR - INVALID INSTRCUTION CODE/ZERO LENGTH FIELD`

---

## 7. Type IDs

The interpretation and parsing of the contents of **Data** fields is not mandated by the IDT Specification, instead, it is defined by the application and identified with a **Data Type ID**.

It is recommended to generate the data type ID by writing a short, unique string describing the data type and any important information about the method of translating that data type too and from its IDT expression. Then, Compute a 32‑bit MurmurHash3 (seed 0) of that string. The resulting 32‑bit value, in big-endian order, becomes the **Data Type ID**.

Keep the original description string, along with its hash, in the application’s source repository. It is recommended to keep the strings in a separate, non-source code file, but use a constant variable for the hash value.

Applications should check that the data structures being filled with data from an IDT file are compatible with the **Data Type IDs** of the **Tables** being read.

If the translation procedure changes, a new string and hash should be created to reflect the change and ensure that IDT data is not read into incompatible data structures.

## 2.1 Header Errors  

| Condition | Error Code | Message |
|-----------|------------|---------|
| **Magic** bytes are not `0x49 44 54` | `IDT FORMAT ERROR` | `INVALID FORMAT IDENTIFIER` |
| **Version** field is unknown or unsupported | `IDT FORMAT ERROR` | `UNSUPPORTED FORMAT VERSION` |
| **Common Field Size (n)** is not a supported value | `IDT FORMAT ERROR` | `UNSUPPORTED INDEX/LENGTH FIELD SIZE` |
| **Validation Block Size (k)** is not supported (e.g., exceeds the implementation limit) | `IDT FORMAT ERROR` | `UNSUPPORTED VALIDATION BLOCK SIZE` |
| **Header parity** (byte 7) does not equal the XOR of bytes 0‑6 | `IDT FORMAT ERROR` | `HEADER VALIDATION FAILURE` |
| **EOF** occurs before all 8 header bytes are read | `IDT FORMAT ERROR` | `INCOMPLETE HEADER` |

---

## Appendix A – Consolidated Error Table  

| Category | Error Code | Message |
|----------|------------|---------|
| **Header** | `IDT FORMAT ERROR` | *see **2.1 Header Errors** table* |
| **Validation** | `IDT VALIDATION ERROR` | `INCOMPLETE VALIDATION BLOCK` |
| **Metadata** | `IDT METADATA ERROR` | *see **5.2 Metadata Errors** table* |
| **Parsing** | `IDT PARSING ERROR` | `DUPLICATE INDEX` |
|  | `IDT PARSING ERROR` | `INVALID CODE/ZERO LENGTH FIELD` |
|  | `IDT PARSING ERROR` | `UNKNOWN INSTRUCTION CODE` |
|  | `IDT PARSING ERROR` | `RESERVED INSTRUCTION CODE (0xFFFF)` |
|  | `IDT PARSING ERROR` | `UNEXPECTED END‑OF‑FILE` |
|  | `IDT PARSING ERROR` | `UNEXPECTED DATA FOUND AFTER STOP FILE INSTRUCTION` |

*All other error messages remain unchanged and are listed in the respective sections.*

---

## Appendix B – Implementation Tips  

1. **Streaming parser skeleton**  
   ```text
   read_header()
   if k > 0:   // validation blocks are present
       while not EOF:
           block = read_validation_block(k)
           if not validate_block(block): abort
           feed_clean_bytes(block.data_region)   // de‑interleaved stream
   else:
       feed_clean_bytes(remaining_file_bytes)

   // now we have a clean byte stream (no parity bytes)
   parse_general_metadata()
   while not end_of_stream:
       parse_table()
   ```

2. **Validation‑block handling**  
   * Read `k² + 2k + 1` bytes in one block.  
   * Compute row‑parity, column‑parity, and overall parity.  
   * If a single‑bit error is detected, locate the offending bit by XOR’ing the mismatched row and column indices; flip that bit and continue.  
   * If more than one bit‑plane reports an error, treat the block as corrupted and abort with `IDT VALIDATION ERROR`.

3. **Padding detection**  
   * After an `END FILE` instruction (`0x00 00` after a `0` where a **Length** was expected), verify that every remaining byte equals `0xFF`.  
   * If a non‑padding byte is found, raise `IDT PARSING ERROR - DATA FOUND AFTER STOP_FILE INSTRUCTION`.

4. **Buffering across block boundaries**  
   * Maintain a mutable `bytearray` called `pending_data`.  
   * When a Table Entry’s **Data** field spills past the current block’s data region, append the available portion to `pending_data` and continue reading the next block’s data region until the required length is satisfied.

5. **Index uniqueness check**  
   * Use a `set` (or hash table) per table to store seen indices.  
   * On each new entry, test membership; on duplicate, abort with `IDT PARSING ERROR - DUPLICATE INDEX`.

6. **Testing strategy**  
   * Generate exhaustive test vectors covering:  
     - `n = 1, 2, 4, 8`  
     - `k = 0, 1, 2, 4, 8` (including the edge case `k = 0` – no validation)  
     - Minimal and maximal General Metadata (`m = 24` and a large value)  
     - Tables with a single entry, many entries, and entries that cross validation‑block boundaries.  
   * For each vector, verify that:  
     - The parser accepts a well‑formed file.  
     - Corrupting a single data byte causes automatic correction.  
     - Corrupting two bits in the same bit‑plane causes a detectable error.  
     - Violations of any error condition produce the exact prescribed message.

7. **Versioning and forward compatibility**  
   * Store a human‑readable version string in the optional “Additional” metadata.  
   * When encountering an unknown **Version** value, reject the file unless the application explicitly opts‑in to “experimental” handling.