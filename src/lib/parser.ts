enum Punc {
  PERIOD = "。",
  COMMA = "，",
  DUNCOMMA = "、",
  COLON = "：",
  EXCL = "！",
  LEFTPAREN = "（",
  RIGHTPAREN = "）",
  QUESTION = "？",
}

const LINEBR_TAG = "<br/>";

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

type Linebreak = {
  type: "linebr";
};

function isPunc(str: string): str is Punc {
  return (
    str === Punc.PERIOD ||
    str === Punc.COMMA ||
    str === Punc.DUNCOMMA ||
    str === Punc.COLON ||
    str === Punc.EXCL ||
    str === Punc.LEFTPAREN ||
    str === Punc.RIGHTPAREN ||
    str === Punc.QUESTION
  );
}

/** Returns a list of parsed characters (hanzi/punctuation) from the mantram text. */
export function parseMantramText(
  str: string,
): (Hanzi | Punctuation | Linebreak)[] {
  return str.split("\n\n").flatMap((line) => {
    const lineTriple = line.split("\n");

    const subtitles = lineTriple[0]
      .split(/\.?,?:?!?\s|\.|!|:|\?/)
      .filter(Boolean);
    const hanzis = lineTriple[1];
    const pinyins = lineTriple[2]
      ? lineTriple[2].split(/\.?,?:?!?\s|\.|!|:|\?/).filter(Boolean)
      : undefined;

    let i = 0;
    let inLineBrBlock = false;
    let inParenBlock = false;
    let res = hanzis
      .split("")
      .map((hanzi) => {
        // NOTE: currently ALL `<tag_name>` tags are assumed to be linebreak tags.
        if (hanzi === LINEBR_TAG[0]) {
          inLineBrBlock = true;
          return null;
        } else if (hanzi === LINEBR_TAG[LINEBR_TAG.length - 1]) {
          inLineBrBlock = false;
          return {
            type: "linebr",
          };
        }
        if (inLineBrBlock) {
          return null;
        }

        if (hanzi === Punc.LEFTPAREN) {
          inParenBlock = true;
        } else if (hanzi === Punc.RIGHTPAREN) {
          inParenBlock = false;
        }

        if (inParenBlock || isPunc(hanzi)) {
          return {
            type: "punc",
            char: hanzi as Punc,
          };
        } else {
          return {
            type: "hanzi",
            char: hanzi,
            sub: subtitles[pinyins ? i : i++],
            pinyin: pinyins ? pinyins[i++] : undefined,
          };
        }
      })
      .filter(Boolean) as (Hanzi | Punctuation | Linebreak)[];

    return res;
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
