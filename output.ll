; ModuleID = 'Entry'
source_filename = "Entry"

define i64 @fibo(i64 %a) {
entry:
  %a2 = alloca i64
  store i64 0, i64* %a2
  %a3 = load i64, i64* %a2
  %a1 = alloca i64
  store i64 %a, i64* %a1

loop:                                             ; preds = %murge
  %a4 = load i64, i64* %a2
  %addtmp = add i64 %a4, 1
  store i64 %addtmp, i64* %a2
  %a5 = load i64, i64* %a2
  %comptmp = icmp eq i64 %a5, 10
  %ifcondition = icmp ne i1 %comptmp, false
  br i1 %ifcondition, label %then, label %else

then:                                             ; preds = %loop
  br label %endloop
  br label %murge

else:                                             ; preds = %loop

murge:                                            ; preds = %then
  %iftmp = phi i64 [ 0, %then ], [ 0, %then ]
  br label %loop
  ret i64 0

endloop:                                          ; preds = %then
}
