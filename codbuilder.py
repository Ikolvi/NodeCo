with open("language_spec/examples/gui.kbj", "wb") as f:
    f.write(bytes([
        0x4E, 0x43, 0x4F, 0x01,      # Magic + version
        0x10, 0x01, 0x00, 0x01, 0x01, 0x2A,  # Create Button (type=1, id=0, 1 property: Text=42)
        0x12, 0x00, 0x01, 0x01,      # On Click (element 0), handler 1
        0x13                         # Show UI
    ]))