+++
title = "Memory layout"
description = "Memory layout of data structures shared with the runtime"
date = 2022-01-15T18:00:00+00:00
updated = 2022-01-15T18:00:00+00:00
draft = false
weight = 20
sort_by = "weight"
template = "docs/page.html"

[extra]
lead = "Memory layout of data structures shared with the runtime"
toc = true
top = false
+++

## Exports
The application must provide the following exports for the host to call: 
<table>
  <thead>
    <tr>
      <th>Name</th>
      <th>Type</th>
      <th>Description</th>
    </tr>
  </thead>
  <tbody>
    <tr>
      <td>OS</td>
      <td><a href="#os">OS</a></td>
      <td>This contains all state shared by the host and the app. It's used by the app to check for inputs and update the screen.</td>
    </tr>
    <tr>
      <td>init</td>
      <td><pre>(os: *os) => *void</pre></td>
      <td>The host calls this function when the WebAssembly module is initialised. The first argument is a pointer to the shared OS data. It can return a pointer to any app specific state, which will then be passed by the host to the <code>frame</code> function.</td>
    </tr>
    <tr>
      <td>frame</td>
      <td><pre>(os: *os, app_state: *void, dt: f64) => void</pre></td>
        <td>This function is called by the host to request a new frame. The first argument contains the pointer to the shared OS data, the second argument contains a pointer to any app specific state returned by <code>init</code>, and the third parameter is a double precision float containing the amount of time passed since the previous frame. In a browser this will typically be called at 60 FPS however it can vary.</td>
    </tr>
  </tbody>
</table>

## Types
### OS
<table>
  <thead>
  <tr>
    <th>Offset (hex)</th>
    <th>Offset (decimal)</th>
    <th>Type</th>
    <th>Size (bytes)</th>
    <th>Name</th>
    <th>Description</th>
  </tr>
  </thead>
  <tbody>
  <tr>
    <td>0x000000</td>
    <td>0</td>
    <td><a href="#config">Config</a></td>
    <td>256</td>
    <td>Config</td>
    <td>See <a href="#config">Config</a> section below</td>
  </tr>
  <tr>
    <td>0x000100</td>
    <td>256</td>
    <td>u8[]</td>
    <td>256</td>
    <td>Inputs</td>
    <td>Each byte in this range corresponds to the state of an input. For keypresses, the value will be set to 1 when the key is initially pressed down, and reset to 0 when the key is released. You may want to reset the value to zero if you only want to capture the initial event, that way you'll receive further keyboard repeats.</td>
  </tr>
  <tr>
    <td>0x000200</td>
    <td>512</td>
    <td></td>
    <td>2560</td>
    <td>Reserved</td>
    <td></td>
  </tr>

  </tr>
    <td>0x000C00</td>
    <td>3072</td>
    <td><a href="#screen-data">Screen Data</a></td>
    <td>65536</td>
    <td>Characters</td>
    <td>Specifies the index of the character to display in a given cell. Full-width characters use two bytes, where the first character must be set to 255 to tell the renderer that it's a full-width character, and the following byte specifies the index of the full-width character to display.</td>
  </tr>

  <tr>
    <td>0x010C00</td>
    <td>68608</td>
    <td><a href="#screen-data">Screen Data</a></td>
    <td>65536</td>
    <td>Background Color</td>
    <td>Specifies the index of the palette colour to use as the background for this cell.</td>
  </tr>

  <tr>
    <td>0x020C00</td>
    <td>134144</td>
    <td><a href="#screen-data">Screen Data</a></td>
    <td>65536</td>
    <td>Foreground Color</td>
    <td>Specifies the index of the palette colour to use as the foreground for this cell.</td>
  </tr>
  </tbody>
</table>

### Config
<table>
  <thead>
    <tr>
      <th>Offset (hex)</th>
      <th>Offset (decimal)</th>
      <th>Type</th>
      <th>Size (bytes)</th>
      <th>Name</th>
      <th>Description</th>
    </tr>
  </thead>
  <tbody>
    <tr>
      <td>0x000000</td>
      <td>0</td>
      <td>u8</td>
      <td>1</td>
      <td>Columns</td>
      <td>The number of horizontal cells to display</td>
    </tr>
    <tr>
      <td>0x000001</td>
      <td>1</td>
      <td>u8</td>
      <td>1</td>
      <td>Rows</td>
      <td>The number of vertical cells to display</td>
    </tr>
    <tr>
      <td>0x000002</td>
      <td>2</td>
      <td>u8</td>
      <td>1</td>
      <td>Max Columns</td>
      <td>(Set by the runtime) The maximum number of horizontal cells that can be displayed. Can change if the user resizes their window.</td>
    </tr>
    <tr>
      <td>0x000003</td>
      <td>3</td>
      <td>u8</td>
      <td>1</td>
      <td>Max Rows</td>
      <td>(Set by the runtime) The maximum number of vertical cells that can be displayed. Can change if the user resizes their window.</td>
    </tr>
    <tr>
      <td>0x000004</td>
      <td>4</td>
      <td></td>
      <td>252</td>
      <td>Reserved</td>
      <td></td>
    </tr>
  </tbody>
</table>

### Screen Data
<table>
  <thead>
    <tr>
      <th>Offset (hex)</th>
      <th>Offset (decimal)</th>
      <th>Type</th>
      <th>Size (bytes)</th>
      <th>Name</th>
      <th>Description</th>
    </tr>
  </thead>
  <tbody>
    <tr>
      <td>0x000000</td>
      <td>0</td>
      <td>u8</td>
      <td>1</td>
      <td>Update Flag</td>
      <td>The application must set this value to 1 if it has updated any of the values in this struct since the previous frame. The host will reset this to 0 when it uploads the data to the GPU. If the data is not updated between frames, it's best to leave it at zero so the host can avoid doing unnecessary work (and saving the user's battery).</td>
    </tr>
    <tr>
      <td>0x000001</td>
      <td>1</td>
      <td>u8[]</td>
      <td>65535</td>
      <td>Cell data</td>
      <td>Each byte of this array represents the associated value for a cell on screen, whether that's ASCII code or palette index. The cells are scanned first in the x direction, then line by line. For example for an 80x30 screen, these values would represent the positions for
        <pre>[(0, 0), (1, 0), (2, 0), .... (80, 0), (0, 1), (1, 1), ...]</pre></td>
    </tr>
  </tbody>
</table>
