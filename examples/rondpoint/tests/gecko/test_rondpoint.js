add_task(async function test_rondpoint() {
  deepEqual(
    Rondpoint.copieDictionnaire({
      un: "deux",
      deux: false,
      petitNombre: 0,
      grosNombre: 123456789,
    }),
    {
      un: "deux",
      deux: false,
      petitNombre: 0,
      grosNombre: 123456789,
    }
  );
  equal(Rondpoint.copieEnumeration("deux"), "deux");
  deepEqual(Rondpoint.copieEnumerations(["un", "deux"]), ["un", "deux"]);
  deepEqual(
    Rondpoint.copieCarte({
      1: "un",
      2: "deux",
    }),
    {
      1: "un",
      2: "deux",
    }
  );
  ok(Rondpoint.switcheroo(false));
});

add_task(async function test_retourneur() {
  let rt = new Retourneur();

  // Booleans.
  [true, false].forEach(v => strictEqual(rt.identiqueBoolean(v), v));

  // Bytes.
  [-128, 127].forEach(v => equal(rt.identiqueI8(v), v));
  [0x00, 0xff].forEach(v => equal(rt.identiqueU8(v), v));

  // Shorts.
  [-Math.pow(2, 15), Math.pow(2, 15) - 1].forEach(v =>
    equal(rt.identiqueI16(v), v)
  );
  [0, 0xffff].forEach(v => equal(rt.identiqueU16(v), v));

  // Ints.
  [0, 1, -1, -Math.pow(2, 31), Math.pow(2, 31) - 1].forEach(v =>
    equal(rt.identiqueI32(v), v)
  );
  [0, Math.pow(2, 32) - 1].forEach(v => equal(rt.identiqueU32(v), v));

  // Longs.
  [0, 1, -1, Number.MIN_SAFE_INTEGER, Number.MAX_SAFE_INTEGER].forEach(v =>
    equal(rt.identiqueI64(v), v)
  );
  [0, 1, Number.MAX_SAFE_INTEGER].forEach(v => equal(rt.identiqueU64(v), v));

  // Floats.
  [0, 1, 0.25].forEach(v => equal(rt.identiqueFloat(v), v));

  // Doubles.
  [0, 1, 0.25].forEach(v => equal(rt.identiqueDouble(v), v));

  // Strings.
  [
    "",
    "abc",
    "été",
    "ښي لاس ته لوستلو لوستل",
    "😻emoji 👨‍👧‍👦multi-emoji, 🇨🇭a flag, a canal, panama",
  ].forEach(v => equal(rt.identiqueString(v), v));
});

add_task(async function test_stringifier() {
  let st = new Stringifier();

  let wellKnown = st.wellKnownString("firefox");
  equal(wellKnown, "uniffi 💚 firefox!");

  let table = {
    toStringBoolean: [
      [true, "true"],
      [false, "false"],
    ],
    toStringI8: [
      [-128, "-128"],
      [127, "127"],
    ],
    toStringU8: [
      [0x00, "0"],
      [0xff, "255"],
    ],
    toStringI16: [
      [-Math.pow(2, 15), "-32768"],
      [Math.pow(2, 15) - 1, "32767"],
    ],
    toStringU16: [
      [0, "0"],
      [0xffff, "65535"],
    ],
    toStringI32: [
      [0, "0"],
      [1, "1"],
      [-1, "-1"],
      [-Math.pow(2, 31), "-2147483648"],
      [Math.pow(2, 31) - 1, "2147483647"],
    ],
    toStringU32: [
      [0, "0"],
      [Math.pow(2, 32) - 1, "4294967295"],
    ],
    toStringI64: [
      [0, "0"],
      [1, "1"],
      [-1, "-1"],
      [Number.MIN_SAFE_INTEGER, "-9007199254740991"],
      [Number.MAX_SAFE_INTEGER, "9007199254740991"],
    ],
    toStringU64: [
      [0, "0"],
      [1, "1"],
      [Number.MAX_SAFE_INTEGER, "9007199254740991"],
    ],
    toStringFloat: [
      [0, "0"],
      [1, "1"],
      [0.25, "0.25"],
    ],
    toStringDouble: [
      [0, "0"],
      [1, "1"],
      [0.25, "0.25"],
    ],
  };
  for (let method in table) {
    for (let [v, expected] of table[method]) {
      strictEqual(st[method](v), expected);
    }
  }
});
