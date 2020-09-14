; ModuleID = 'Entry'
source_filename = "Entry"

define i64 @foo(i64 %0) {
entry:
  %a = alloca i64
  store i64 0, i64* %a
  %a1 = load i64, i64* %a
  %1 = alloca i64
  store i64 %0, i64* %1

loop:                                             ; preds = %murge
  %a2 = load i64, i64* %a
  %comptmp = icmp eq i64 %a2, 10
  %ifcondition = icmp ne i1 %comptmp, false
  br i1 %ifcondition, label %then, label %else

then:                                             ; preds = %loop
  br label %endloop

else:                                             ; preds = %loop

murge:                                            ; preds = %endloop, %endloop
  %iftmp = phi i64 [ 0, %endloop ], [ 10, %endloop ]
  %a3 = load i64, i64* %a
  %addtmp = add i64 %a3, 1
  store i64 %addtmp, i64* %a
  br label %loop
  ret i64 %addtmp

endloop:                                          ; preds = %then
  br label %murge
  br label %murge
}
