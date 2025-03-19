#include <fstream>
#include <iostream>
#include <string>
#include <unordered_map>
#include <vector>
using namespace std;

// step 2->2. node created
struct Node {
  char ch;
  int freq;
  Node *left, *right;

  Node(char ch, int freq, Node *left = nullptr, Node *right = nullptr)
      : ch(ch), freq(freq), left(left), right(right) {}
};

struct Compare {
  bool operator()(Node *a, Node *b) { return a->freq > b->freq; }
};

// step 2->3. tree creation/popilation
void serializeTree(Node *root, string &treeStructure) {
  if (!root)
    return;
  if (!root->left && !root->right) {
    treeStructure += "1";
    treeStructure += root->ch;
  } else {
    treeStructure += "0";
    serializeTree(root->left, treeStructure);
    serializeTree(root->right, treeStructure);
  }
}

// step 3: use the tree to generate the prefix-code table.
void buildCodeTable(Node *root, string code,
                    unordered_map<char, string> &huffmanCode) {
  if (!root)
    return;
  if (!root->left && !root->right)
    huffmanCode[root->ch] = code;
  buildCodeTable(root->left, code + "0", huffmanCode);
  buildCodeTable(root->right, code + "1", huffmanCode);
}

int main() {
  // step 1:
  // open the file
  cout << "Enter the filename: ";
  string filename;
  cin >> filename;
  // read the file
  ifstream file(filename);
  // exception handling
  if (!file) {
    cerr << "Error: Unable to open file!" << endl;
    return 1;
  }

  string text;
  char c;
  while (file.get(c)) {
    text += c;
  }
  file.close();

  // determine frequency of each character
  // use a map with char assigned to a frequency integer
  unordered_map<char, int> freq;

  for (char ch : text)
    freq[ch]++; // read whole file char by char

  // step 2:
  // build a binary tree using frequency map
  // Huffman tree node implementation: Base class
  // Huffman tree node: Leaf class
  // Huffman tree node: Internal class
  // A Huffman coding tree
  // implement tree building process
  priority_queue<Node *, vector<Node *>, Compare> pq;
  for (auto &pair : freq)
    pq.push(new Node(pair.first, pair.second));

  while (pq.size() > 1) {
    Node *left = pq.top();
    pq.pop();
    Node *right = pq.top();
    pq.pop();
    pq.push(new Node('\0', left->freq + right->freq, left, right));
  }

  Node *root = pq.top();

  // step 3: creation of prefix-code table
  unordered_map<char, string> huffmanCode;
  buildCodeTable(root, "", huffmanCode);

  string encodedText = "";
  for (char ch : filename)
    encodedText += huffmanCode[ch];

  string treeStructure = "";
  serializeTree(root, treeStructure);

  // step 4: write a header section to the output file
  ofstream outFile("encoded.txt");
  outFile << treeStructure << '\n' << encodedText;
  outFile.close();

  cout << "Encoded text and Huffman tree written to file." << endl;
  return 0;
}
