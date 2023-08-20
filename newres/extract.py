import os
import json

def load_words():
    """Load words from dictionary words/*.json"""
    words = []
    for filename in os.listdir('words'):
        if filename.endswith('json'):
            print(f'Loading {filename}...')
            with open('words/' + filename) as f:
                print(f'Reading {filename}...')
                words += json.loads(f.read()).keys()
    return words

def main():
    words = load_words()
    print(f'Loaded {len(words)} words.')
    words = sorted(set(map(str.lower, words)))
    print(f'Unique words: {len(words)}')
    with open('words.txt', 'w') as f:
        f.write('\n'.join(words))
    print('Done.')

if __name__ == '__main__':
    main()