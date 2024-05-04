enum Punc {
  PERIOD = "。",
  COMMA = "，",
  DUNCOMMA = "、",
  COLON = "：",
}

type Hanzi = {
  type: "hanzi";
  char: string;
  sub: string;
  pinyin: string | undefined;
};

type Punctuation = {
  type: "punc";
  char: Punc;
};

function isPunc(str: string): str is Punc {
  return (
    str === Punc.PERIOD ||
    str === Punc.COMMA ||
    str === Punc.DUNCOMMA ||
    str === Punc.COLON
  );
}

/** Returns a list of parsed characters (hanzi/punctuation) from the mantram text. */
export function parseMantramText(str: string): (Hanzi | Punctuation)[] {
  return str.split("\n\n").flatMap((line) => {
    const lineTriple = line.split("\n");

    const subtitles = lineTriple[0].split(/\.?,?:?\s|\./).filter(Boolean);
    const hanzis = lineTriple[1];
    const pinyins = lineTriple[2]
      ? lineTriple[2].split(/\.?,?:?\s|\./).filter(Boolean)
      : undefined;

    let i = 0;
    return hanzis.split("").map((hanzi) => {
      if (isPunc(hanzi)) {
        return {
          type: "punc",
          char: hanzi,
        };
      } else {
        return {
          type: "hanzi",
          char: hanzi,
          sub: subtitles[pinyins ? i : i++],
          pinyin: pinyins ? pinyins[i++] : undefined,
        };
      }
    });
  });
}

/** Returns a list of parsed hanzi chars from the supplied hanzi-sub-pinyin combination. */
export function parseHanziCombo({
  hanzis,
  subs,
  pinyins,
}: {
  hanzis: string;
  subs: string;
  pinyins: string | undefined;
}): Hanzi[] {
  const sbs = subs.split(" ");
  const hzs = hanzis.split("");
  const pys = pinyins ? pinyins.split(" ") : undefined;

  return hzs.map((hanzi, i) => {
    return {
      type: "hanzi",
      char: hanzi,
      sub: sbs[i],
      pinyin: pys ? pys[i] : undefined,
    };
  });
}
