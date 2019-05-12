# Troubleshooting

## Failed to parse ELF sections while calculating the virtual size
This happens when the ELF sections are more than 99.
This might occur when using `core::str::from_utf8` (an alternative would be `from_utf8_unchecked`), `core::fmt` or `alloc::str(ing)`.
The Pebble SDK will break if the digits in the first column of the `readelf` command are more than 2.

To fix this, open `~/.pebble-sdk/SDKs/current/sdk-core/pebble/common/tools/inject_metadata.py`,
then change line `136` from
```py
line = line[6:]
```
to
```py
if not ']' in line:
    continue
line=line[line.index(']')+1:]
```
