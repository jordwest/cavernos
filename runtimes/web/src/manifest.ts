export type ManifestV1 = {
  manifestVersion: 1;
  program: string;
  fonts: {
    gridSize: { width: number; height: number };
    narrow: string;
    square: string;
  };
  palette: string;
  postProcessing?: boolean;
};
