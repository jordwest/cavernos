export type ManifestV1 = {
  manifestVersion: 1;
  program: string;
  fonts: {
    gridSize: { width: number; height: number };
    narrow: string;
    square: string;
  };
  palette: string;
  inputMappings: InputMappingV1[];
};

type InputMappingV1 = {
  /**
   * The address to which this input's value will be written
   */
  index: number;

  /**
   * An optional comment describing this key
   */
  comment?: string;

  /**
   * The keys that will trigger this input
   */
  keys: string[];
};
