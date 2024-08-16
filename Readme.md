# Operation Encoding Documentation

### 1. **SETDATA (0x01)**

This operation sets a portion of data at a specific offset.


+--------+-------------+----------------+
| Opcode | Data Offset |  New Data      |
+--------+-------------+----------------+
| 0x01   | 2 bytes     |  Variable size |
+--------+-------------+----------------+


- **Opcode (0x01)**: 1 byte (SETDATA)
- **Data Offset**: 2 bytes (starting position where new data will be written)
- **New Data**: Variable size (data to insert into the buffer)

---

### 2. **RESETDATA (0x02)**

This operation resets the buffer to a new size.


+--------+---------------+
| Opcode | New Data Size |
+--------+---------------+
| 0x02   | 2 bytes       |
+--------+---------------+


- **Opcode (0x02)**: 1 byte (RESETDATA)
- **New Data Size**: 2 bytes (size of the new buffer)

---

### 3. **SETVALUE (0x03)**

This operation sets the value to be sent in the next CALL/CREATE operation.


+--------+----------+
| Opcode | Value    |
+--------+----------+
| 0x03   | 32 bytes |
+--------+----------+


- **Opcode (0x03)**: 1 byte (SETVALUE)
- **Value**: 32 bytes (amount of Ether to send)

---

### 4. **CALL (0x04)**

This operation makes a call using the constructed transaction info.


+--------+----------------+
| Opcode | Target Address |
+--------+----------------+
| 0x04   | 20 bytes       |
+--------+----------------+


- **Opcode (0x04)**: 1 byte (CALL)
- **Target Address**: 20 bytes (address of the contract to call)

---

### 5. **CREATE (0x05)**

This operation creates a contract using the constructed transaction info.


+--------+
| Opcode |
+--------+
| 0x05   |
+--------+


- **Opcode (0x05)**: 1 byte (CREATE)

---

### 6. **EXTCODECOPY (0x06)**

This operation copies external contract code into the buffer.


+--------+----------------+-------------+-------------+
| Opcode | Source Address | Code Offset | Copy Length |
+--------+----------------+-------------+-------------+
| 0x06   | 20 bytes       | 2 bytes     | 2 bytes     |
+--------+----------------+-------------+-------------+


- **Opcode (0x06)**: 1 byte (EXTCODECOPY)
- **Source Address**: 20 bytes (address of the external contract)
- **Code Offset**: 2 bytes (offset in the code to start copying from)
- **Copy Length**: 2 bytes (length of the code to copy)

---

### 7. **PATCH (0x07)**

This operation applies a list of patches to the buffer.


+--------+-------------------+
| Opcode | Number of Patches |
+--------+-------------------+
| 0x07   | 1 byte            |
+--------+-------------------+


- **Opcode (0x07)**: 1 byte (PATCH)
- **Number of Patches**: 1 byte (number of patches to apply)

---

### 8. **SETTARGET (0x08)**

This operation sets the target address for subsequent CALL/CREATE operations.


+--------+-----------------+
| Opcode | Target Address  |
+--------+-----------------+
| 0x08   | 20 bytes        |
+--------+-----------------+


- **Opcode (0x08)**: 1 byte (SETTARGET)
- **Target Address**: 20 bytes (address to set as the target)

---

### 9. **SETALLOWFAIL (0x09)**

This operation sets the allowFail flag for subsequent CALL/CREATE operations.


+--------+------------+
| Opcode | Allow Fail |
+--------+------------+
| 0x09   | 1 byte     |
+--------+------------+


- **Opcode (0x09)**: 1 byte (SETALLOWFAIL)
- **Allow Fail**: 1 byte (flag to allow or disallow failure)


---

# Error Code Documentation

The contract returns single-byte error codes to minimize bytecode size. Below is a table that maps each error code to a corresponding error description.

| Error Code | Description                       |
|------------|-----------------------------------|
| 0x01       | Invalid Opcode                    |
| 0x02       | Buffer Overflow                   |
| 0x03       | Insufficient Funds for Operation  |
| 0x04       | Invalid Target Address            |
| 0x05       | Operation Failed                  |
| 0x06       | Contract Creation Failed          |
| 0x07       | Unauthorized Access               |
| 0x08       | Memory Overflow                   |

Each operation can trigger these revert errors depending on the conditions met during execution.