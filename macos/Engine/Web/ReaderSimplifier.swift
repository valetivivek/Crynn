import WebKit

enum ReaderSimplifier {
    static func applySimplify(in webView: WKWebView) {
        let js = """
        (function(){
          function getMain(){
            var sel=['article','main','#content','.post','.entry','.article'];
            for (var i=0;i<sel.length;i++){var n=document.querySelector(sel[i]); if(n){return n;}}
            return document.body;
          }
          var n=getMain();
          var s=document.createElement('style');
          s.textContent='body{font: -apple-system-headline; margin:0; background:#111; color:#eee} .crynn{max-width: 740px; margin:40px auto; padding:0 20px; line-height:1.6; font-size:18px} img,video{max-width:100%; height:auto} a{color:#9bd}';
          var html='<!doctype html><meta name="viewport" content="width=device-width, initial-scale=1"><title>'+document.title+'</title>';
          html+=s.outerHTML;
          html+='<div class="crynn">'+(n.innerHTML||'')+'</div>';
          document.open(); document.write(html); document.close();
        })();
        """
        webView.evaluateJavaScript(js) { _, _ in }
    }
}


