# Numbers - Binary Number System

## Decimal Solution

**NOTE** : The 2 is in 1's place & 4 is in 10's place, we used 10 as base cause in decimal we have 10 distinct digits from 0 to 9. The exponent is 0 so in mathematics there is a rule that whichever base it is if the exponent is 0 the base will become 1.

42

| 4 | 2 |

| 10<sup>1</sup> | 10<sup>0</sup> |

(4 x 10<sup>1</sup>) + (2 x 10<sup>0</sup>)
= (4 x 10) + (2 x 1)
= 40 + 2
= 42

## Binary Solution

42

| 0 | 0 | 1 | 0 | 1 | 0 | 1 | 0 |

| 2<sup>7</sup> | 2<sup>6</sup> | 2<sup>5</sup> | 2<sup>4</sup> | 2<sup>3</sup> | 2<sup>2</sup> | 2<sup>1</sup> | 2<sup>0</sup> |

| 128 | 64 | 32 | 16 | 8 | 4 | 2 | 1 |

(1 x 2<sup>5</sup>) + (1 x 2<sup>3</sup>) + (1 x 2<sup>1</sup>)
= (1 x 32) + (1 x 8) + (1 x 2)
= 40 + 2
= 42

**NOTE** : The base is 2 cause in binary there is only 1 & 0. So we will just multiply the binary numbers that are 1.

## Numbers - Range of 8-bit integers

### Smallest possible 8-bit integer (unsigned): 0

| 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |

| 2<sup>7</sup> | 2<sup>6</sup> | 2<sup>5</sup> | 2<sup>4</sup> | 2<sup>3</sup> | 2<sup>2</sup> | 2<sup>1</sup> | 2<sup>0</sup> |

| 128 | 64 | 32 | 16 | 8 | 4 | 2 | 1 |

### Largest possible 8-bit integer (unsigned): 255

| 1 | 1 | 1 | 1 | 1 | 1 | 1 | 1 |

| 2<sup>7</sup> | 2<sup>6</sup> | 2<sup>5</sup> | 2<sup>4</sup> | 2<sup>3</sup> | 2<sup>2</sup> | 2<sup>1</sup> | 2<sup>0</sup> |

| 128 | 64 | 32 | 16 | 8 | 4 | 2 | 1 |

## Numbers - Range of 16-bit integers

### Smallest possible 16-bit integer (unsigned): 0

| 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |

| 2<sup>15</sup> | 2<sup>14</sup> | 2<sup>13</sup> | 2<sup>12</sup> | 2<sup>11</sup> | 2<sup>10</sup> | 2<sup>9</sup> | 2<sup>8</sup> | 2<sup>7</sup> | 2<sup>6</sup> | 2<sup>5</sup> | 2<sup>4</sup> | 2<sup>3</sup> | 2<sup>2</sup> | 2<sup>1</sup> | 2<sup>0</sup> |

| 32768 | 16384 | 8192 | 4096 | 2048 | 1024 | 512 | 256 | 128 | 64 | 32 | 16 | 8 | 4 | 2 | 1 |

### Smallest possible 16-bit integer (unsigned): 65535

| 1 | 1 | 1 | 1 | 1 | 1 | 1 | 1 | 1 | 1 | 1 | 1 | 1 | 1 | 1 | 1 |

| 2<sup>15</sup> | 2<sup>14</sup> | 2<sup>13</sup> | 2<sup>12</sup> | 2<sup>11</sup> | 2<sup>10</sup> | 2<sup>9</sup> | 2<sup>8</sup> | 2<sup>7</sup> | 2<sup>6</sup> | 2<sup>5</sup> | 2<sup>4</sup> | 2<sup>3</sup> | 2<sup>2</sup> | 2<sup>1</sup> | 2<sup>0</sup> |

| 32768 | 16384 | 8192 | 4096 | 2048 | 1024 | 512 | 256 | 128 | 64 | 32 | 16 | 8 | 4 | 2 | 1 |

**NOTE** : We have to minus 1 in 65536 cause the range starts from 1 here 'cause of exponent, but in reality it starts from 0 so we have to minus 1 in this range so it will become 65535.
