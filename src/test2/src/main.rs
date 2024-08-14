use minidom::Element;

fn main() {
    let str = "<?xml version=\'1.0\' encoding=\'utf-8\'?>
<package xmlns=\"http://www.idpf.org/2007/opf\" version=\"3.0\" unique-identifier=\"uuid_id\" prefix=\"calibre: https://calibre-ebook.com\">
  <metadata xmlns:opf=\"http://www.idpf.org/2007/opf\" xmlns:dc=\"http://purl.org/dc/elements/1.1/\" xmlns:dcterms=\"http://purl.org/dc/terms/\" xmlns:xsi=\"http://www.w3.org/2001/XMLSchema-instance\" xmlns:calibre=\"http://calibre.kovidgoyal.net/2009/metadata\">
    <dc:title id=\"id\" xml:lang=\"zh\">败北女角太多了！ 5</dc:title>
    <dc:creator id=\"id-1\">雨森焚火</dc:creator>
    <dc:rights>epub制作者：kid(天使动漫论坛)</dc:rights>
    <dc:identifier>calibre:2</dc:identifier>
    <dc:identifier>uuid:7d6a13bb-ac94-4f15-ade3-192779b5583e</dc:identifier>
    <dc:identifier id=\"uuid_id\">uuid:7d6a13bb-ac94-4f15-ade3-192779b5583e</dc:identifier>
    <dc:language>zh</dc:language>
    <dc:contributor id=\"id-2\">calibre (7.16.0) [https://calibre-ebook.com]</dc:contributor>
    <dc:date>2023-08-05T16:00:00+00:00</dc:date>
    <dc:subject>轻小说</dc:subject>
    <meta refines=\"#id\" property=\"title-type\">main</meta>
    <meta refines=\"#id\" property=\"file-as\">败北女角太多了！ 5</meta>
    <meta property=\"calibre:timestamp\" scheme=\"dcterms:W3CDTF\">2024-08-07T03:45:31Z</meta>
    <meta property=\"dcterms:modified\" scheme=\"dcterms:W3CDTF\">2024-08-13T04:09:43Z</meta>
    <meta refines=\"#id-2\" property=\"role\" scheme=\"marc:relators\">bkp</meta>
    <meta refines=\"#id-1\" property=\"role\" scheme=\"marc:relators\">aut</meta>
    <meta refines=\"#id-1\" property=\"file-as\">kid</meta>
  </metadata>
  <manifest>
    <item id=\"titlepage\" href=\"titlepage.xhtml\" media-type=\"application/xhtml+xml\" properties=\"svg calibre:title-page\"/>
    <item id=\"title.xhtml\" href=\"OEBPS/Text/title.xhtml\" media-type=\"application/xhtml+xml\"/>
    <item id=\"message.xhtml\" href=\"OEBPS/Text/message.xhtml\" media-type=\"application/xhtml+xml\"/>
    <item id=\"summary.xhtml\" href=\"OEBPS/Text/summary.xhtml\" media-type=\"application/xhtml+xml\"/>
    <item id=\"illus1.xhtml\" href=\"OEBPS/Text/illus1.xhtml\" media-type=\"application/xhtml+xml\"/>
    <item id=\"illus2.xhtml\" href=\"OEBPS/Text/illus2.xhtml\" media-type=\"application/xhtml+xml\"/>
    <item id=\"illus3.xhtml\" href=\"OEBPS/Text/illus3.xhtml\" media-type=\"application/xhtml+xml\"/>
    <item id=\"illus4.xhtml\" href=\"OEBPS/Text/illus4.xhtml\" media-type=\"application/xhtml+xml\"/>
    <item id=\"contents-illus.xhtml\" href=\"OEBPS/Text/contents-illus.xhtml\" media-type=\"application/xhtml+xml\"/>
    <item id=\"contents.xhtml\" href=\"OEBPS/Text/contents.xhtml\" media-type=\"application/xhtml+xml\"/>
    <item id=\"intro.xhtml\" href=\"OEBPS/Text/intro.xhtml\" media-type=\"application/xhtml+xml\"/>
    <item id=\"Section000.xhtml\" href=\"OEBPS/Text/Section000.xhtml\" media-type=\"application/xhtml+xml\"/>
    <item id=\"Section001.xhtml\" href=\"OEBPS/Text/Section001.xhtml\" media-type=\"application/xhtml+xml\"/>
    <item id=\"Intermission-1.xhtml\" href=\"OEBPS/Text/Intermission-1.xhtml\" media-type=\"application/xhtml+xml\"/>
    <item id=\"Section002.xhtml\" href=\"OEBPS/Text/Section002.xhtml\" media-type=\"application/xhtml+xml\"/>
    <item id=\"Intermission-2.xhtml\" href=\"OEBPS/Text/Intermission-2.xhtml\" media-type=\"application/xhtml+xml\"/>
    <item id=\"Section003.xhtml\" href=\"OEBPS/Text/Section003.xhtml\" media-type=\"application/xhtml+xml\"/>
    <item id=\"Intermission-3.xhtml\" href=\"OEBPS/Text/Intermission-3.xhtml\" media-type=\"application/xhtml+xml\"/>
    <item id=\"Section004.xhtml\" href=\"OEBPS/Text/Section004.xhtml\" media-type=\"application/xhtml+xml\"/>
    <item id=\"Section005.xhtml\" href=\"OEBPS/Text/Section005.xhtml\" media-type=\"application/xhtml+xml\"/>
    <item id=\"Postscript.xhtml\" href=\"OEBPS/Text/Postscript.xhtml\" media-type=\"application/xhtml+xml\"/>
    <item id=\"Section006.xhtml\" href=\"OEBPS/Text/Section006.xhtml\" media-type=\"application/xhtml+xml\"/>
    <item id=\"Special-illus.xhtml\" href=\"OEBPS/Text/Special-illus.xhtml\" media-type=\"application/xhtml+xml\"/>
    <item id=\"Special.xhtml\" href=\"OEBPS/Text/Special.xhtml\" media-type=\"application/xhtml+xml\"/>
    <item id=\"Bookmark1.xhtml\" href=\"OEBPS/Text/Bookmark1.xhtml\" media-type=\"application/xhtml+xml\"/>
    <item id=\"Bookmark2.xhtml\" href=\"OEBPS/Text/Bookmark2.xhtml\" media-type=\"application/xhtml+xml\"/>
    <item id=\"backcover.xhtml\" href=\"OEBPS/Text/backcover.xhtml\" media-type=\"application/xhtml+xml\"/>
    <item id=\"nav\" href=\"nav.xhtml\" media-type=\"application/xhtml+xml\" properties=\"nav\"/>
    <item id=\"page_css\" href=\"page_styles.css\" media-type=\"text/css\"/>
    <item id=\"css\" href=\"stylesheet.css\" media-type=\"text/css\"/>
    <item id=\"cover\" href=\"cover.jpeg\" media-type=\"image/jpeg\" properties=\"cover-image\"/>
    <item id=\"x001.jpg\" href=\"OEBPS/Images/001.jpg\" media-type=\"image/jpeg\"/>
    <item id=\"x002.jpg\" href=\"OEBPS/Images/002.jpg\" media-type=\"image/jpeg\"/>
    <item id=\"x003.jpg\" href=\"OEBPS/Images/003.jpg\" media-type=\"image/jpeg\"/>
    <item id=\"x004.jpg\" href=\"OEBPS/Images/004.jpg\" media-type=\"image/jpeg\"/>
    <item id=\"x005.jpg\" href=\"OEBPS/Images/005.jpg\" media-type=\"image/jpeg\"/>
    <item id=\"x006.jpg\" href=\"OEBPS/Images/006.jpg\" media-type=\"image/jpeg\"/>
    <item id=\"x007.jpg\" href=\"OEBPS/Images/007.jpg\" media-type=\"image/jpeg\"/>
    <item id=\"x008.jpg\" href=\"OEBPS/Images/008.jpg\" media-type=\"image/jpeg\"/>
    <item id=\"x009.jpg\" href=\"OEBPS/Images/009.jpg\" media-type=\"image/jpeg\"/>
    <item id=\"x010.jpg\" href=\"OEBPS/Images/010.jpg\" media-type=\"image/jpeg\"/>
    <item id=\"x011.jpg\" href=\"OEBPS/Images/011.jpg\" media-type=\"image/jpeg\"/>
    <item id=\"x012.jpg\" href=\"OEBPS/Images/012.jpg\" media-type=\"image/jpeg\"/>
    <item id=\"x013.jpg\" href=\"OEBPS/Images/013.jpg\" media-type=\"image/jpeg\"/>
    <item id=\"bc.jpg\" href=\"OEBPS/Images/bc.jpg\" media-type=\"image/jpeg\"/>
    <item id=\"co1.jpg\" href=\"OEBPS/Images/co1.jpg\" media-type=\"image/jpeg\"/>
    <item id=\"co2.jpg\" href=\"OEBPS/Images/co2.jpg\" media-type=\"image/jpeg\"/>
    <item id=\"co3.jpg\" href=\"OEBPS/Images/co3.jpg\" media-type=\"image/jpeg\"/>
    <item id=\"co4.jpg\" href=\"OEBPS/Images/co4.jpg\" media-type=\"image/jpeg\"/>
    <item id=\"co5.jpg\" href=\"OEBPS/Images/co5.jpg\" media-type=\"image/jpeg\"/>
    <item id=\"co6.jpg\" href=\"OEBPS/Images/co6.jpg\" media-type=\"image/jpeg\"/>
    <item id=\"note.png\" href=\"OEBPS/Images/note.png\" media-type=\"image/png\"/>
    <item id=\"t1.png\" href=\"OEBPS/Images/t1.png\" media-type=\"image/png\"/>
    <item id=\"illus1.ttf\" href=\"OEBPS/Fonts/illus1.ttf\" media-type=\"application/vnd.ms-opentype\"/>
    <item id=\"title.ttf\" href=\"OEBPS/Fonts/title.ttf\" media-type=\"application/vnd.ms-opentype\"/>
  </manifest>
  <spine>
    <itemref idref=\"titlepage\"/>
    <itemref idref=\"title.xhtml\"/>
    <itemref idref=\"message.xhtml\"/>
    <itemref idref=\"summary.xhtml\"/>
    <itemref idref=\"illus1.xhtml\"/>
    <itemref idref=\"illus2.xhtml\"/>
    <itemref idref=\"illus3.xhtml\"/>
    <itemref idref=\"illus4.xhtml\"/>
    <itemref idref=\"contents-illus.xhtml\"/>
    <itemref idref=\"contents.xhtml\"/>
    <itemref idref=\"intro.xhtml\"/>
    <itemref idref=\"Section000.xhtml\"/>
    <itemref idref=\"Section001.xhtml\"/>
    <itemref idref=\"Intermission-1.xhtml\"/>
    <itemref idref=\"Section002.xhtml\"/>
    <itemref idref=\"Intermission-2.xhtml\"/>
    <itemref idref=\"Section003.xhtml\"/>
    <itemref idref=\"Intermission-3.xhtml\"/>
    <itemref idref=\"Section004.xhtml\"/>
    <itemref idref=\"Section005.xhtml\"/>
    <itemref idref=\"Postscript.xhtml\"/>
    <itemref idref=\"Section006.xhtml\"/>
    <itemref idref=\"Special-illus.xhtml\"/>
    <itemref idref=\"Special.xhtml\"/>
    <itemref idref=\"Bookmark1.xhtml\"/>
    <itemref idref=\"Bookmark2.xhtml\"/>
    <itemref idref=\"backcover.xhtml\"/>
  </spine>
</package>
    ";

    let elem = str.parse::<Element>().unwrap();

    println!("{:?}", elem.prefixes);

    let elem = elem.children().next().unwrap();

    println!("{:?}", elem.prefixes);

    let elem = elem.children().next().unwrap();

    println!("{:?}", elem.prefixes);



    // let data = str.as_bytes();
    //
    // minidom::Element::from_reader_with_prefixes(
    //     BufReader::new(data),
    // )
}