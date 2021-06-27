interface Padding {
  unpadded: number,
  padded: number
}

export function getRowPadding(width: number): Padding {
  // It is a webgpu requirement that BufferCopyView.layout.bytes_per_row % COPY_BYTES_PER_ROW_ALIGNMENT(256) == 0
  // So we calculate padded_bytes_per_row by rounding unpadded_bytes_per_row
  // up to the next multiple of COPY_BYTES_PER_ROW_ALIGNMENT.
  // https://en.wikipedia.org/wiki/Data_structure_alignment#Computing_padding
  const bytesPerPixel = 4;
  const unpaddedBytesPerRow = width * bytesPerPixel;
  const align = 256;
  const paddedBytesPerRowPadding = (align - unpaddedBytesPerRow % align) % align;
  const paddedBytesPerRow = unpaddedBytesPerRow + paddedBytesPerRowPadding;
  return {
    unpadded: unpaddedBytesPerRow,
    padded: paddedBytesPerRow,
  };
}